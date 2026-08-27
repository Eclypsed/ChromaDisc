//! Attribute + item parsing for `#[derive(BitsEnum)]`.
//!
//! Turns a raw [`DeriveInput`] into a [`BitsSpec`] the analysis and codegen
//! phases can consume. All errors point at user source with spans preserved.
//!
//! Grammar (PLAN §5):
//!
//! ```text
//! #[bits(<width>, repr = <ty>, reserved = <pat>, field = "…")]  // enum
//! #[bits(alt = <pat>, fallback)]                                 // variant
//! ```

use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, Ident, LitInt, LitStr, Pat, Token, Type, Variant,
};

/// Parsed enum-level state.
pub(crate) struct BitsSpec {
    pub name: Ident,
    pub width: u32,
    pub repr: Type,
    pub reserved: Option<Pat>,
    pub field_name: String,
    pub variants: Vec<VariantSpec>,
}

pub(crate) struct VariantSpec {
    pub ident: Ident,
    /// The RHS of `= <expr>`. Required for every variant.
    pub discriminant: Expr,
    pub alt: Option<Pat>,
    /// `Some(span)` if the variant was marked `#[bits(fallback)]`.
    pub fallback_span: Option<Span>,
}

pub(crate) fn parse_bits_spec(input: &DeriveInput) -> syn::Result<BitsSpec> {
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        Data::Struct(_) | Data::Union(_) => {
            return Err(syn::Error::new(
                input.ident.span(),
                "`BitsEnum` can only be derived on fieldless enums",
            ));
        }
    };

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "`BitsEnum` does not currently support generic enums",
        ));
    }

    let enum_attrs = collect_entries(&input.attrs)?;
    let mut width: Option<u32> = None;
    let mut repr: Option<(Type, Span)> = None;
    let mut reserved: Option<Pat> = None;
    let mut field_name: Option<String> = None;

    for entry in enum_attrs {
        match entry {
            Entry::Width(lit) => {
                let span = lit.span();
                let value: u32 = lit
                    .base10_parse()
                    .map_err(|e| syn::Error::new(span, format!("invalid field width: {e}")))?;
                if value == 0 {
                    return Err(syn::Error::new(span, "field width must be at least 1"));
                }
                if value > 128 {
                    return Err(syn::Error::new(
                        span,
                        "field width must be at most 128 bits",
                    ));
                }
                if width.replace(value).is_some() {
                    return Err(syn::Error::new(span, "field width specified twice"));
                }
            }
            Entry::Repr(ty) => {
                let span = ty.span();
                if repr.replace((ty, span)).is_some() {
                    return Err(syn::Error::new(span, "`repr = ...` specified twice"));
                }
            }
            Entry::Reserved(pat) => {
                if reserved.replace(pat).is_some() {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "`reserved = ...` specified twice",
                    ));
                }
            }
            Entry::Field(lit) => {
                if field_name.replace(lit.value()).is_some() {
                    return Err(syn::Error::new(lit.span(), "`field = ...` specified twice"));
                }
            }
            Entry::Alt(pat) => {
                return Err(syn::Error::new(
                    pat.span(),
                    "`alt = ...` is a variant-level key; put it on a variant",
                ));
            }
            Entry::Fallback(span) => {
                return Err(syn::Error::new(
                    span,
                    "`fallback` is a variant-level key; put it on a variant",
                ));
            }
        }
    }

    let width = width.ok_or_else(|| {
        syn::Error::new(
            input.ident.span(),
            "missing field width; add `#[bits(<width>)]` to the enum",
        )
    })?;

    let repr = match repr {
        Some((ty, _)) => ty,
        None => default_repr(width, Span::call_site()),
    };

    let field_name = field_name.unwrap_or_else(|| input.ident.to_string());

    let variants = variants
        .iter()
        .map(parse_variant)
        .collect::<syn::Result<Vec<_>>>()?;

    // `fallback` and `reserved` don't compose: the fallback absorbs the
    // complement, so asserting its contents is meaningless (PLAN §6.3).
    if let (Some(fallback_span), Some(_)) = (
        variants.iter().find_map(|v| v.fallback_span),
        reserved.as_ref(),
    ) {
        return Err(syn::Error::new(
            fallback_span,
            "`#[bits(fallback)]` cannot be combined with an enum-level \
             `reserved = ...`: the fallback absorbs the complement so \
             asserting it is meaningless",
        ));
    }

    Ok(BitsSpec {
        name: input.ident.clone(),
        width,
        repr,
        reserved,
        field_name,
        variants,
    })
}

fn parse_variant(v: &Variant) -> syn::Result<VariantSpec> {
    match &v.fields {
        Fields::Unit => {}
        Fields::Named(_) | Fields::Unnamed(_) => {
            return Err(syn::Error::new(
                v.fields.span(),
                "`BitsEnum` variants must be fieldless: see PLAN §3.3 for why",
            ));
        }
    }

    let discriminant = match &v.discriminant {
        Some((_, expr)) => expr.clone(),
        None => {
            return Err(syn::Error::new(
                v.ident.span(),
                format!(
                    "variant `{}` needs a discriminant; add `= 0b…` or `= 0x…`",
                    v.ident
                ),
            ));
        }
    };

    let entries = collect_entries(&v.attrs)?;
    let mut alt: Option<Pat> = None;
    let mut fallback_span: Option<Span> = None;

    for entry in entries {
        match entry {
            Entry::Alt(pat) => {
                if alt.replace(pat).is_some() {
                    return Err(syn::Error::new(
                        v.ident.span(),
                        "`alt = ...` specified twice on the same variant",
                    ));
                }
            }
            Entry::Fallback(span) => {
                if fallback_span.replace(span).is_some() {
                    return Err(syn::Error::new(span, "`fallback` specified twice"));
                }
            }
            Entry::Width(lit) => {
                return Err(syn::Error::new(
                    lit.span(),
                    "field width is an enum-level key; put it on the enum",
                ));
            }
            Entry::Repr(ty) => {
                return Err(syn::Error::new(
                    ty.span(),
                    "`repr = ...` is an enum-level key; put it on the enum",
                ));
            }
            Entry::Reserved(pat) => {
                return Err(syn::Error::new(
                    pat.span(),
                    "`reserved = ...` is an enum-level key; put it on the enum",
                ));
            }
            Entry::Field(lit) => {
                return Err(syn::Error::new(
                    lit.span(),
                    "`field = ...` is an enum-level key; put it on the enum",
                ));
            }
        }
    }

    Ok(VariantSpec {
        ident: v.ident.clone(),
        discriminant,
        alt,
        fallback_span,
    })
}

/// Union of every kind of thing that can appear inside a `#[bits(...)]`
/// meta-list. Splitting into enum-level vs variant-level is a validation
/// concern, not a syntactic one.
enum Entry {
    Width(LitInt),
    Repr(Type),
    Reserved(Pat),
    Field(LitStr),
    Alt(Pat),
    Fallback(Span),
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Positional integer literal → width.
        if input.peek(LitInt) && !input.peek2(Token![=]) {
            return Ok(Entry::Width(input.parse()?));
        }
        let key: Ident = input.parse()?;
        let key_str = key.to_string();
        match key_str.as_str() {
            "fallback" => Ok(Entry::Fallback(key.span())),
            "repr" | "reserved" | "field" | "alt" => {
                input.parse::<Token![=]>().map_err(|_| {
                    syn::Error::new(key.span(), format!("`{key_str}` requires `= <value>`"))
                })?;
                match key_str.as_str() {
                    "repr" => Ok(Entry::Repr(input.parse()?)),
                    "field" => Ok(Entry::Field(input.parse()?)),
                    "reserved" => Ok(Entry::Reserved(Pat::parse_multi_with_leading_vert(input)?)),
                    "alt" => Ok(Entry::Alt(Pat::parse_multi_with_leading_vert(input)?)),
                    _ => unreachable!(),
                }
            }
            _ => Err(syn::Error::new(
                key.span(),
                format!(
                    "unknown `#[bits(...)]` key `{key_str}`; expected one of: \
                     repr, reserved, field, alt, fallback"
                ),
            )),
        }
    }
}

fn collect_entries(attrs: &[Attribute]) -> syn::Result<Vec<Entry>> {
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("bits") {
            continue;
        }
        let parser = Punctuated::<Entry, Token![,]>::parse_terminated;
        let entries = attr.parse_args_with(parser)?;
        out.extend(entries.into_iter());
    }
    Ok(out)
}

/// Smallest unsigned primitive that can hold `width` bits.
fn default_repr(width: u32, span: Span) -> Type {
    let name = match width {
        1..=8 => "u8",
        9..=16 => "u16",
        17..=32 => "u32",
        33..=64 => "u64",
        65..=128 => "u128",
        _ => unreachable!("width already bounded to 1..=128"),
    };
    let ident = Ident::new(name, span);
    syn::parse2(ident.into_token_stream()).expect("primitive ident is a valid Type")
}
