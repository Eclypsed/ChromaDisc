//! Procedural macro backing the [`bitfield`] crate.
//!
//! Everything user-facing lives in [`bitfield`]; this crate is only a
//! compilation dependency and should never be depended on directly. See
//! `bitfield/PLAN.md` §2 for the two-crate rationale.
//!
//! [`bitfield`]: https://docs.rs/bitfield

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod analyse;
mod emit;
mod parse;

/// Derive [`bitfield::BitsRepr`] and companion inherent methods for a
/// fieldless enum. See `bitfield`'s crate docs for the full attribute
/// grammar.
#[proc_macro_derive(BitsEnum, attributes(bits))]
pub fn derive_bits_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_impl(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let spec = parse::parse_bits_spec(&input)?;
    let analysed = analyse::analyse(spec)?;
    Ok(emit::emit(analysed))
}
