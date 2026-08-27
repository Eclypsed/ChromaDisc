//! Code emission for `#[derive(BitsEnum)]`.
//!
//! Consumes [`AnalysedSpec`] and produces the exact set of impls PLAN §4.2,
//! §7, §8.1, and §9 permit at 1.0 — nothing more, nothing less.
//!
//! Emitted items:
//! - inherent `const fn from_bits` and `const fn to_bits`
//! - `impl BitsRepr for T`
//! - `impl From<T> for <repr>`
//! - `impl From<<repr>> for T` **iff** the mapping is total *and* the repr
//!   width equals the field width (PLAN §7 last paragraph)

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Pat, Type};

use crate::analyse::{AnalysedSpec, AnalysedVariant, intervals_from_pat};

pub(crate) fn emit(spec: AnalysedSpec) -> TokenStream {
    let crate_path = resolve_crate_path();
    let enum_name = &spec.name;
    let repr = &spec.repr;
    let width = spec.width;
    let field_name = &spec.field_name;

    let primitive = smallest_primitive(width);
    let repr_kind = classify_repr(repr, &primitive);

    let (to_prim, from_prim) = repr_adapters(repr, &repr_kind, &primitive);

    let match_arms = build_match_arms(&spec.variants, enum_name, width);
    let needs_wildcard = needs_wildcard(&spec, &repr_kind);

    let from_bits_doc = build_from_bits_doc(&spec);
    let to_bits_doc = build_to_bits_doc(&spec);

    let wildcard_arm = if needs_wildcard {
        quote! { _ => ::core::option::Option::None }
    } else {
        // No wildcard needed: exhaustive match on a repr whose value set is
        // fully covered by explicit arms.
        TokenStream::new()
    };

    // `,` between last arm and wildcard when both exist
    let arms_sep = if needs_wildcard {
        quote! { , }
    } else {
        TokenStream::new()
    };

    let inherent_from_bits = quote! {
        #[doc = #from_bits_doc]
        #[allow(deprecated)]
        pub const fn from_bits(bits: #repr) -> ::core::option::Option<Self> {
            match #to_prim {
                #match_arms #arms_sep #wildcard_arm
            }
        }
    };

    let inherent_to_bits = quote! {
        #[doc = #to_bits_doc]
        #[allow(deprecated)]
        pub const fn to_bits(self) -> #repr {
            #from_prim
        }
    };

    let bits_repr_impl = quote! {
        #[automatically_derived]
        #[allow(deprecated)]
        impl #crate_path::BitsRepr for #enum_name {
            type Bits = #repr;
            const BITS: u32 = #width;
            const FIELD: &'static str = #field_name;

            #[inline]
            fn from_bits(bits: Self::Bits) -> ::core::option::Option<Self> {
                Self::from_bits(bits)
            }

            #[inline]
            fn to_bits(self) -> Self::Bits {
                Self::to_bits(self)
            }
        }
    };

    let from_enum_impl = quote! {
        #[automatically_derived]
        #[allow(deprecated)]
        impl ::core::convert::From<#enum_name> for #repr {
            #[inline]
            fn from(v: #enum_name) -> Self {
                v.to_bits()
            }
        }
    };

    let from_repr_impl = if spec.total && repr_kind.field_matches_repr(width) {
        let msg = format!("`from_bits` for `{enum_name}` is total; this is unreachable",);
        quote! {
            #[automatically_derived]
            #[allow(deprecated)]
            impl ::core::convert::From<#repr> for #enum_name {
                #[inline]
                fn from(v: #repr) -> Self {
                    match Self::from_bits(v) {
                        ::core::option::Option::Some(x) => x,
                        // Analysis proved every pattern is claimed.
                        ::core::option::Option::None => ::core::panic!(#msg),
                    }
                }
            }
        }
    } else {
        TokenStream::new()
    };

    quote! {
        impl #enum_name {
            #inherent_from_bits
            #inherent_to_bits
        }

        #bits_repr_impl
        #from_enum_impl
        #from_repr_impl
    }
}

// -- crate-path resolution ---------------------------------------------------

fn resolve_crate_path() -> TokenStream {
    // We deliberately do NOT use `crate::` for the `FoundCrate::Itself` case.
    // `proc-macro-crate` returns `Itself` for examples and integration tests
    // within the support crate's package, where `crate::` refers to the
    // example binary rather than the library. Emitting `::bitfield::` and
    // adding `extern crate self as bitfield;` in the library's own lib.rs
    // covers all three call sites (downstream user, in-library tests,
    // in-package examples) with the same tokens.
    match crate_name("bitfield") {
        Ok(FoundCrate::Itself) | Err(_) => quote! { ::bitfield },
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
    }
}

// -- repr classification -----------------------------------------------------

enum ReprKind {
    /// A raw Rust primitive: `u8`, `u16`, …
    Primitive(Ident),
    /// A wrapper such as `arbitrary_int::u4`. We treat any non-primitive as a
    /// wrapper and generate `.value()` / `::new(...)` accordingly (PLAN §7).
    Wrapper,
}

impl ReprKind {
    fn field_matches_repr(&self, field_width: u32) -> bool {
        match self {
            // For a primitive repr, "field matches repr" means the primitive
            // is exactly the width of the field. Otherwise there are always
            // out-of-range values that decode to None.
            ReprKind::Primitive(ident) => {
                let repr_bits = primitive_bit_width(ident);
                repr_bits == Some(field_width)
            }
            // For a wrapper, we assume the wrapper's exact width equals the
            // field width. That's the whole point of writing `repr = u4` on
            // a 4-bit field.
            ReprKind::Wrapper => true,
        }
    }
}

fn classify_repr(repr: &Type, primitive: &Ident) -> ReprKind {
    if let Some(ident) = as_single_ident(repr)
        && primitive_bit_width(&ident).is_some()
    {
        return ReprKind::Primitive(ident);
    }
    // Not a bare primitive ident → treat as a wrapper. The default repr is
    // always a primitive, so if we get here the user wrote something.
    let _ = primitive;
    ReprKind::Wrapper
}

fn as_single_ident(ty: &Type) -> Option<Ident> {
    if let Type::Path(path) = ty
        && path.qself.is_none()
        && path.path.segments.len() == 1
    {
        let seg = &path.path.segments[0];
        if seg.arguments.is_empty() {
            return Some(seg.ident.clone());
        }
    }
    None
}

fn primitive_bit_width(ident: &Ident) -> Option<u32> {
    match ident.to_string().as_str() {
        "u8" => Some(8),
        "u16" => Some(16),
        "u32" => Some(32),
        "u64" => Some(64),
        "u128" => Some(128),
        // `usize` isn't stable-width and never a sensible repr for a wire
        // field. Treat it as "not a match" for totality purposes, but still
        // allow it as a primitive for adapter classification.
        _ => None,
    }
}

fn smallest_primitive(width: u32) -> Ident {
    let name = match width {
        1..=8 => "u8",
        9..=16 => "u16",
        17..=32 => "u32",
        33..=64 => "u64",
        65..=128 => "u128",
        _ => unreachable!("width already bounded to 1..=128"),
    };
    Ident::new(name, Span::call_site())
}

// -- adapters ---------------------------------------------------------------

/// Returns `(to_prim, from_prim)` — the expression evaluated inside the
/// `match` scrutinee, and the expression used to construct the repr from
/// `self`.
///
/// `primitive` is the smallest unsigned type that holds the field width;
/// for wrapper reprs, arbitrary_int's `::new` takes exactly that type and
/// `.value()` returns exactly that type.
fn repr_adapters(repr: &Type, kind: &ReprKind, primitive: &Ident) -> (TokenStream, TokenStream) {
    match kind {
        ReprKind::Primitive(ident) => (quote! { bits }, quote! { self as #ident }),
        ReprKind::Wrapper => (
            quote! { bits.value() },
            quote! { <#repr>::new(self as #primitive) },
        ),
    }
}

// -- match arms -------------------------------------------------------------

fn build_match_arms(
    variants: &[AnalysedVariant],
    enum_name: &Ident,
    width: u32,
) -> TokenStream {
    let mut arms = TokenStream::new();
    let mut first = true;

    // Explicit variants first, fallback last so its `_` never shadows an
    // explicit arm.
    for v in variants.iter().filter(|v| !v.is_fallback) {
        if !first {
            arms.extend(quote! { , });
        }
        first = false;
        let ident = &v.ident;
        let pattern = build_variant_pattern(v, width);
        arms.extend(quote! {
            #pattern => ::core::option::Option::Some(#enum_name::#ident)
        });
    }

    if let Some(v) = variants.iter().find(|v| v.is_fallback) {
        if !first {
            arms.extend(quote! { , });
        }
        let ident = &v.ident;
        arms.extend(quote! {
            _ => ::core::option::Option::Some(#enum_name::#ident)
        });
    }

    arms
}

/// Emit the match-arm pattern for one non-fallback variant.
///
/// The straightforward `disc | alt` layout can trigger clippy's
/// `almost_complete_range` / `manual_range_patterns` lints in downstream
/// crates when `disc` and `alt` happen to be contiguous — e.g.
/// `0b1011 | 0b1100..=0b1101` should really be `0b1011..=0b1101`.
///
/// We fix this at the `disc`-boundary only: alt cases contiguous with the
/// discriminant get folded into a single range that spans them.
/// Adjacencies purely within `alt` (like the user writing
/// `alt = 0b010..=0b011 | 0b100..=0b101`) are preserved as-written so the
/// lint keeps firing — that adjacency is on the user, not on the derive.
fn build_variant_pattern(v: &AnalysedVariant, width: u32) -> TokenStream {
    let disc_lit = &v.discriminant_lit;
    let disc_val = v.intervals[0].0; // discriminant always occupies intervals[0]

    let Some(alt) = &v.alt else {
        return quote! { #disc_lit };
    };

    let max = if width == 128 { u128::MAX } else { (1u128 << width) - 1 };
    let cases = flatten_or_toplevel(alt);
    let (merged, kept) = merge_contiguous(disc_val, cases, width, max);

    // No merge happened: preserve the user's alt tokens verbatim.
    if merged == (disc_val, disc_val) {
        return quote! { #disc_lit | #alt };
    }

    let (lo, hi) = merged;
    // Zero-pad to the field width so the generated pattern reads consistently
    // — matches the width users would write themselves (e.g. `0b0101` on a
    // 4-bit field rather than `0b101`). `+ 2` covers the `0b` prefix.
    let w = width as usize + 2;
    let merged_tokens: TokenStream = format!("{lo:#0w$b}..={hi:#0w$b}", w = w)
        .parse()
        .expect("binary range literal is valid Rust");
    let mut result = merged_tokens;
    for case in kept {
        result.extend(quote! { | #case });
    }
    result
}

/// Split a top-level `Or` pattern into its cases, flattening through parens.
/// Non-`Or` patterns become a single-element list.
fn flatten_or_toplevel(pat: &Pat) -> Vec<Pat> {
    match pat {
        Pat::Or(pat_or) => pat_or
            .cases
            .iter()
            .flat_map(flatten_or_toplevel)
            .collect(),
        Pat::Paren(paren) => flatten_or_toplevel(&paren.pat),
        other => vec![other.clone()],
    }
}

/// Repeatedly extend the merged range with any single-interval case
/// contiguous to its current bounds. Cases that don't touch (or that
/// resolve to multiple sub-intervals) are kept in their original order.
fn merge_contiguous(
    disc_val: u128,
    cases: Vec<Pat>,
    width: u32,
    max: u128,
) -> ((u128, u128), Vec<Pat>) {
    let mut merged = (disc_val, disc_val);
    let mut kept = cases;
    loop {
        let mut still_kept = Vec::with_capacity(kept.len());
        let mut merged_this_pass = false;
        for case in kept {
            match single_interval(&case, width, max) {
                Some((lo, hi)) if merged.0 > 0 && hi + 1 == merged.0 => {
                    merged.0 = lo;
                    merged_this_pass = true;
                }
                Some((lo, hi)) if merged.1 < max && merged.1 + 1 == lo => {
                    merged.1 = hi;
                    merged_this_pass = true;
                }
                _ => still_kept.push(case),
            }
        }
        kept = still_kept;
        if !merged_this_pass {
            break;
        }
    }
    (merged, kept)
}

fn single_interval(pat: &Pat, width: u32, max: u128) -> Option<(u128, u128)> {
    let ivs = intervals_from_pat(pat, width, max).ok()?;
    if ivs.len() == 1 {
        Some(ivs[0])
    } else {
        None
    }
}

/// A wildcard `_ => None` is needed when:
/// - there is no fallback variant, AND
/// - explicit variants cover fewer patterns than the repr can express.
///
/// A primitive repr wider than the field always leaves out-of-range values,
/// so this is always true for `repr = u8` on a 4-bit field. A wrapper repr
/// whose width equals the field width can, in principle, be exhaustively
/// covered, but the match runs on the underlying primitive (which is
/// *wider*), so we still need the wildcard for the compiler's exhaustiveness
/// check.
fn needs_wildcard(spec: &AnalysedSpec, _repr_kind: &ReprKind) -> bool {
    // A fallback variant already provides a `_` arm.
    !spec.variants.iter().any(|v| v.is_fallback)
}

// -- doc comments -----------------------------------------------------------

fn build_from_bits_doc(spec: &AnalysedSpec) -> String {
    let mut out = format!(
        "Decode a raw pattern into a variant of [`{}`](Self).\n\nReturns \
         `None` for any pattern the mapping does not claim.\n\n\
         # Mappings\n\n",
        spec.name,
    );
    for v in &spec.variants {
        if v.is_fallback {
            out.push_str(&format!(
                "- `{}`: fallback (catches every otherwise-unclaimed pattern)\n",
                v.ident,
            ));
        } else {
            out.push_str(&format!(
                "- `{}`: {}\n",
                v.ident,
                format_intervals_binary(&v.intervals),
            ));
        }
    }
    out
}

fn build_to_bits_doc(spec: &AnalysedSpec) -> String {
    format!(
        "Encode a variant of [`{}`](Self) to its canonical bit pattern.",
        spec.name,
    )
}

fn format_intervals_binary(intervals: &[(u128, u128)]) -> String {
    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|&(lo, _)| lo);
    sorted
        .into_iter()
        .map(|(lo, hi)| {
            if lo == hi {
                format!("`{lo:#b}`")
            } else {
                format!("`{lo:#b}..={hi:#b}`")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}
