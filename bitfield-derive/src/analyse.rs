//! Coverage + width analysis over parsed `#[bits(...)]` state.
//!
//! Consumes [`BitsSpec`] and produces [`AnalysedSpec`] carrying everything
//! codegen needs: canonicalised discriminant values, per-variant interval
//! sets (for docs), and a totality flag that gates whether
//! `impl From<uN> for T` is safe to emit.
//!
//! Algorithms follow PLAN §6 verbatim: patterns → intervals, sort by start,
//! adjacent scan for overlap and gap. No 2^N sentinel array — the scan is
//! O(k log k) in declared ranges (PLAN §6.1).

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Ident, Lit, LitInt, Pat, RangeLimits, Type};

use crate::parse::{BitsSpec, VariantSpec};

pub(crate) struct AnalysedSpec {
    pub name: Ident,
    pub width: u32,
    pub repr: Type,
    pub field_name: String,
    pub variants: Vec<AnalysedVariant>,
    /// True iff every pattern in `0 .. 2^width` is claimed — either by
    /// exhaustive declarations, or by a `#[bits(fallback)]` variant.
    pub total: bool,
}

pub(crate) struct AnalysedVariant {
    pub ident: Ident,
    /// The variant's canonical encoding, as the user wrote it. Preserved so
    /// codegen can splice the exact token back into the match arm.
    pub discriminant_lit: LitInt,
    /// User-supplied `alt` pattern, preserved for splicing.
    pub alt: Option<Pat>,
    /// Every interval this variant claims, closed on both ends. Includes the
    /// discriminant. Sorted, non-empty when `!is_fallback`.
    pub intervals: Vec<(u128, u128)>,
    pub is_fallback: bool,
}

pub(crate) fn analyse(spec: BitsSpec) -> syn::Result<AnalysedSpec> {
    let width = spec.width;
    let max = width_max(width);

    let mut analysed_variants = Vec::with_capacity(spec.variants.len());
    for v in &spec.variants {
        analysed_variants.push(analyse_variant(v, width, max)?);
    }

    // Overlap check runs against explicit declarations regardless of
    // `fallback`. Gap check only fires when the mapping isn't total.
    let has_fallback = analysed_variants.iter().any(|v| v.is_fallback);
    check_overlaps(&analysed_variants)?;

    let declared_gaps = compute_gaps(&analysed_variants, max);

    if !has_fallback && !declared_gaps.is_empty() {
        if let Some(reserved_pat) = spec.reserved.as_ref() {
            // `reserved = ...` is an assertion: computed complement must
            // match declared reserved patterns *exactly* (PLAN §6.2).
            let reserved_intervals = normalise(intervals_from_pat(reserved_pat, width, max)?);
            let declared_normalised = normalise(declared_gaps.clone());
            if reserved_intervals != declared_normalised {
                return Err(syn::Error::new(
                    reserved_pat.span(),
                    format!(
                        "`reserved = ...` disagrees with computed coverage.\n  \
                         declared reserved: {}\n  computed uncovered: {}",
                        format_intervals(&reserved_intervals),
                        format_intervals(&declared_normalised),
                    ),
                ));
            }
        } else {
            return Err(syn::Error::new(
                spec.name.span(),
                format!(
                    "field `{}` has uncovered bit patterns: {}\n  \
                     add variants that claim them, add `#[bits(fallback)]` to \
                     one variant, or add `reserved = ...` to the enum to \
                     assert they are intentionally unclaimed",
                    spec.name,
                    format_intervals(&declared_gaps),
                ),
            ));
        }
    }

    let total = has_fallback || declared_gaps.is_empty();

    Ok(AnalysedSpec {
        name: spec.name,
        width: spec.width,
        repr: spec.repr,
        field_name: spec.field_name,
        variants: analysed_variants,
        total,
    })
}

fn analyse_variant(v: &VariantSpec, width: u32, max: u128) -> syn::Result<AnalysedVariant> {
    let (discriminant_lit, discriminant_value) = eval_discriminant(&v.discriminant, width, max)?;

    let mut intervals = vec![(discriminant_value, discriminant_value)];
    if let Some(alt) = &v.alt {
        // Fallback + alt would ambiguously extend the catch-all. Rare misuse,
        // but worth blocking.
        if v.fallback_span.is_some() {
            return Err(syn::Error::new(
                alt.span(),
                "`alt = ...` on a fallback variant is redundant — the fallback \
                 already claims every unclaimed pattern",
            ));
        }
        intervals.extend(intervals_from_pat(alt, width, max)?);
    }

    Ok(AnalysedVariant {
        ident: v.ident.clone(),
        discriminant_lit,
        alt: v.alt.clone(),
        intervals,
        is_fallback: v.fallback_span.is_some(),
    })
}

/// Detect any overlap between explicitly declared intervals. Fallback
/// variants don't participate — they claim only what's left over.
fn check_overlaps(variants: &[AnalysedVariant]) -> syn::Result<()> {
    let mut all: Vec<(u128, u128, &Ident)> = Vec::new();
    for v in variants {
        if v.is_fallback {
            continue;
        }
        for &(lo, hi) in &v.intervals {
            all.push((lo, hi, &v.ident));
        }
    }
    all.sort_by_key(|&(lo, _, _)| lo);
    for pair in all.windows(2) {
        let (_a_lo, a_hi, a_id) = pair[0];
        let (b_lo, _b_hi, b_id) = pair[1];
        if b_lo <= a_hi {
            let overlap_start = b_lo;
            let overlap_end = a_hi.min(pair[1].1);
            let span = if a_id.span().source_text().is_some() {
                a_id.span()
            } else {
                b_id.span()
            };
            return Err(syn::Error::new(
                span,
                format!(
                    "variants `{a_id}` and `{b_id}` both claim pattern(s) \
                     {}; every pattern can be claimed by at most one variant",
                    format_interval((overlap_start, overlap_end)),
                ),
            ));
        }
    }
    Ok(())
}

/// Compute the complement of explicitly declared intervals over `0..=max`.
fn compute_gaps(variants: &[AnalysedVariant], max: u128) -> Vec<(u128, u128)> {
    let mut declared: Vec<(u128, u128)> = variants
        .iter()
        .filter(|v| !v.is_fallback)
        .flat_map(|v| v.intervals.iter().copied())
        .collect();
    declared.sort_by_key(|&(lo, _)| lo);

    let merged = normalise(declared);
    let mut gaps = Vec::new();
    let mut cursor: u128 = 0;
    for (lo, hi) in merged {
        if lo > cursor {
            gaps.push((cursor, lo - 1));
        }
        // hi + 1 can overflow only if hi == u128::MAX, in which case there
        // can be no gap after it.
        if hi == max {
            return gaps;
        }
        cursor = hi + 1;
    }
    if cursor <= max {
        gaps.push((cursor, max));
    }
    gaps
}

/// Sort + merge adjacent-touching intervals. Idempotent.
fn normalise(mut intervals: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    intervals.sort_by_key(|&(lo, _)| lo);
    let mut out: Vec<(u128, u128)> = Vec::with_capacity(intervals.len());
    for (lo, hi) in intervals {
        match out.last_mut() {
            Some(last) if lo <= last.1.saturating_add(1) => {
                last.1 = last.1.max(hi);
            }
            _ => out.push((lo, hi)),
        }
    }
    out
}

pub(crate) fn intervals_from_pat(
    pat: &Pat,
    width: u32,
    max: u128,
) -> syn::Result<Vec<(u128, u128)>> {
    match pat {
        Pat::Lit(lit) => {
            let value = eval_int_lit(&lit.lit, width, max)?;
            Ok(vec![(value, value)])
        }
        Pat::Or(pat_or) => {
            let mut out = Vec::new();
            for case in &pat_or.cases {
                out.extend(intervals_from_pat(case, width, max)?);
            }
            Ok(out)
        }
        Pat::Range(range) => {
            let start = match &range.start {
                Some(expr) => eval_int_expr(expr, width, max)?,
                None => 0,
            };
            let end = match &range.end {
                Some(expr) => eval_int_expr(expr, width, max)?,
                None => max,
            };
            let (lo, hi) = match &range.limits {
                RangeLimits::HalfOpen(_) => {
                    if end == 0 {
                        return Err(syn::Error::new(
                            range.span(),
                            "empty range: exclusive upper bound is zero",
                        ));
                    }
                    (start, end - 1)
                }
                RangeLimits::Closed(_) => (start, end),
            };
            if lo > hi {
                return Err(syn::Error::new(
                    range.span(),
                    format!("empty range: start ({lo}) is greater than end ({hi})"),
                ));
            }
            Ok(vec![(lo, hi)])
        }
        Pat::Paren(paren) => intervals_from_pat(&paren.pat, width, max),
        _ => Err(syn::Error::new(
            pat.span(),
            "unsupported pattern; expected an integer literal, range, or `|` \
             alternation. Use `#[bits(fallback)]` on a variant if you need a \
             catch-all.",
        )),
    }
}

fn eval_discriminant(expr: &Expr, width: u32, max: u128) -> syn::Result<(LitInt, u128)> {
    let lit = match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(l), ..
        }) => l.clone(),
        _ => {
            return Err(syn::Error::new(
                expr.span(),
                "variant discriminant must be a plain integer literal",
            ));
        }
    };
    let value = parse_int_literal(&lit)?;
    check_width(value, width, max, lit.span())?;
    Ok((lit, value))
}

fn eval_int_expr(expr: &Expr, width: u32, max: u128) -> syn::Result<u128> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(l), ..
        }) => {
            let v = parse_int_literal(l)?;
            check_width(v, width, max, l.span())?;
            Ok(v)
        }
        _ => Err(syn::Error::new(
            expr.span(),
            "expected a plain integer literal",
        )),
    }
}

fn eval_int_lit(lit: &Lit, width: u32, max: u128) -> syn::Result<u128> {
    let Lit::Int(l) = lit else {
        return Err(syn::Error::new(lit.span(), "expected an integer literal"));
    };
    let v = parse_int_literal(l)?;
    check_width(v, width, max, l.span())?;
    Ok(v)
}

/// Width-and-radix-agnostic parse for a syn `LitInt`. `base10_parse` in
/// syn does not handle `0b`/`0x`/`0o` prefixes, so we work off the raw
/// token text.
pub(crate) fn parse_int_literal(lit: &LitInt) -> syn::Result<u128> {
    let text = lit.token().to_string();
    let suffix_len = lit.suffix().len();
    let digits = &text[..text.len() - suffix_len];

    let (radix, digits) = if let Some(rest) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16u32, rest)
    } else if let Some(rest) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2, rest)
    } else if let Some(rest) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8, rest)
    } else {
        (10, digits)
    };

    let cleaned: String = digits.chars().filter(|c| *c != '_').collect();
    u128::from_str_radix(&cleaned, radix)
        .map_err(|e| syn::Error::new(lit.span(), format!("invalid integer literal: {e}")))
}

fn check_width(value: u128, width: u32, max: u128, span: Span) -> syn::Result<()> {
    if value > max {
        return Err(syn::Error::new(
            span,
            format!(
                "`{value:#b}` does not fit in a {width}-bit field \
                 (max value is `{max:#b}`)"
            ),
        ));
    }
    Ok(())
}

fn width_max(width: u32) -> u128 {
    if width == 128 {
        u128::MAX
    } else {
        (1u128 << width) - 1
    }
}

fn format_interval((lo, hi): (u128, u128)) -> String {
    if lo == hi {
        format!("{lo:#b}")
    } else {
        format!("{lo:#b}..={hi:#b}")
    }
}

fn format_intervals(intervals: &[(u128, u128)]) -> String {
    intervals
        .iter()
        .map(|&iv| format_interval(iv))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_pat(text: &str) -> Pat {
        syn::parse::Parser::parse2(Pat::parse_multi_with_leading_vert, text.parse().unwrap())
            .unwrap()
    }

    fn parse_int(text: &str) -> u128 {
        let lit: LitInt = syn::parse_str(text).unwrap();
        parse_int_literal(&lit).unwrap()
    }

    #[test]
    fn int_literal_radices() {
        assert_eq!(parse_int("0"), 0);
        assert_eq!(parse_int("10"), 10);
        assert_eq!(parse_int("0b1010"), 0b1010);
        assert_eq!(parse_int("0x0A"), 0x0A);
        assert_eq!(parse_int("0o12"), 0o12);
        assert_eq!(parse_int("1_000"), 1_000);
        assert_eq!(parse_int("0b1_010u8"), 0b1010);
    }

    #[test]
    fn intervals_from_literal() {
        let pat = parse_pat("0b0010");
        let ivs = intervals_from_pat(&pat, 4, 15).unwrap();
        assert_eq!(ivs, vec![(2, 2)]);
    }

    #[test]
    fn intervals_from_closed_range() {
        let pat = parse_pat("0b0010..=0b0101");
        let ivs = intervals_from_pat(&pat, 4, 15).unwrap();
        assert_eq!(ivs, vec![(2, 5)]);
    }

    #[test]
    fn intervals_from_half_open_range() {
        let pat = parse_pat("2..5");
        let ivs = intervals_from_pat(&pat, 4, 15).unwrap();
        assert_eq!(ivs, vec![(2, 4)]);
    }

    #[test]
    fn intervals_from_or_alternation() {
        let pat = parse_pat("0b0010 | 0b0100..=0b0101 | 0b1000");
        let mut ivs = intervals_from_pat(&pat, 4, 15).unwrap();
        ivs.sort();
        assert_eq!(ivs, vec![(2, 2), (4, 5), (8, 8)]);
    }

    #[test]
    fn width_rejects_overflow() {
        let pat = parse_pat("0b10000");
        let err = intervals_from_pat(&pat, 4, 15).unwrap_err();
        assert!(err.to_string().contains("does not fit in a 4-bit field"));
    }

    #[test]
    fn rejects_bindings() {
        let pat = parse_pat("_");
        let err = intervals_from_pat(&pat, 4, 15).unwrap_err();
        assert!(err.to_string().contains("fallback"));
    }

    #[test]
    fn normalise_merges_adjacent() {
        assert_eq!(
            normalise(vec![(0, 2), (3, 5), (7, 7)]),
            vec![(0, 5), (7, 7)]
        );
        assert_eq!(normalise(vec![(3, 5), (0, 2), (2, 4)]), vec![(0, 5)]);
    }

    /// Property test: brute-force oracle. For every width 1..=8, generate
    /// random declared interval sets and compare the sort-and-scan gap
    /// output against a bitmap complement. PLAN §11 phase 3 calls this out
    /// as the highest-leverage test in the crate.
    #[test]
    fn coverage_matches_brute_force_oracle() {
        use std::collections::BTreeSet;

        // Reproducible pseudo-random via a fixed sequence of seeds.
        for seed in 0..256u32 {
            for width in 1..=8u32 {
                let max = width_max(width);
                let intervals = pseudo_random_intervals(seed, width, max);
                let expected: BTreeSet<u128> = (0..=max)
                    .filter(|v| !intervals.iter().any(|&(lo, hi)| lo <= *v && *v <= hi))
                    .collect();
                let variants = intervals
                    .iter()
                    .enumerate()
                    .map(|(i, &(lo, hi))| AnalysedVariant {
                        ident: syn::Ident::new(&format!("V{i}"), Span::call_site()),
                        discriminant_lit: syn::parse_str::<LitInt>(&lo.to_string()).unwrap(),
                        alt: None,
                        intervals: vec![(lo, hi)],
                        is_fallback: false,
                    })
                    .collect::<Vec<_>>();
                let gaps = compute_gaps(&variants, max);
                let actual: BTreeSet<u128> =
                    gaps.into_iter().flat_map(|(lo, hi)| lo..=hi).collect();
                assert_eq!(
                    actual, expected,
                    "gap mismatch at seed={seed} width={width}: intervals={intervals:?}"
                );
            }
        }
    }

    fn pseudo_random_intervals(seed: u32, width: u32, max: u128) -> Vec<(u128, u128)> {
        // Tiny linear-congruential generator so we don't drag in `rand`.
        let mut state = seed.wrapping_mul(2654435761).wrapping_add(1);
        let mut next = |lo: u128, hi: u128| -> u128 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            lo + (state as u128 % (hi - lo + 1))
        };
        let count = (next(0, 4) as usize).min(1 << width.min(4));
        let mut intervals: Vec<(u128, u128)> = Vec::new();
        for _ in 0..count {
            let a = next(0, max);
            let b = next(0, max);
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            // Skip any interval that would overlap an existing one — this
            // test compares the gap computation only under the assumption
            // that inputs are conflict-free (overlaps are a separate check).
            if intervals.iter().any(|&(l, h)| !(hi < l || lo > h)) {
                continue;
            }
            intervals.push((lo, hi));
        }
        intervals
    }
}
