use proc_macro::TokenStream;
use quote::quote;
use syn::{
    BinOp, Data, DeriveInput, Expr, ExprBinary, ExprLit, ExprRange, Ident, Lit, RangeLimits, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

// TODO: Add duplicate detection?

enum CodeValue {
    Exact(u8),
    OneOf(Vec<u8>),
    Range {
        start: Option<u8>,
        end: Option<u8>,
        limits: RangeLimits,
    },
    WildCard,
}

impl CodeValue {
    fn to_pattern(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Exact(v) => quote!(#v),
            Self::OneOf(vals) => quote!(#(#vals)|*),
            Self::Range { start, end, limits } => match (start, end, limits) {
                (Some(s), Some(e), RangeLimits::HalfOpen(..)) => quote!(#s..#e),
                (Some(s), None, RangeLimits::HalfOpen(..)) => quote!(#s..),
                (None, Some(e), RangeLimits::HalfOpen(..)) => quote!(..#e),
                (None, None, RangeLimits::HalfOpen(..)) => quote!(..),
                (Some(s), Some(e), RangeLimits::Closed(..)) => quote!(#s..=#e),
                (None, Some(e), RangeLimits::Closed(..)) => quote!(..=#e),
                _ => panic!("Invalid range"),
            },
            Self::WildCard => quote!(_),
        }
    }
}

struct KeyValue {
    key: Ident,
    _eq: Token![=],
    value: CodeValue,
}

struct MacroArgs {
    sk: CodeValue,
    asc: CodeValue,
    ascq: CodeValue,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _eq: input.parse()?,
            value: parse_code_value(&input.parse()?),
        })
    }
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_kvs: Punctuated<KeyValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut sk: Option<CodeValue> = None;
        let mut asc: Option<CodeValue> = None;
        let mut ascq: Option<CodeValue> = None;

        for kv in parsed_kvs {
            if kv.key == "sk" {
                if sk.is_some() {
                    panic!("`sk` encounterd more than once");
                }
                sk = Some(kv.value);
            } else if kv.key == "asc" {
                if asc.is_some() {
                    panic!("`asc` encounterd more than once");
                }
                asc = Some(kv.value);
            } else if kv.key == "ascq" {
                if ascq.is_some() {
                    panic!("`ascq encounterd more than once`");
                }
                ascq = Some(kv.value);
            }
        }

        let sk = sk.ok_or_else(|| input.error("missing required key `sk`"))?;
        let asc = asc.ok_or_else(|| input.error("missing required key `asc`"))?;
        let ascq = ascq.ok_or_else(|| input.error("missing required key `ascq`"))?;

        Ok(Self { sk, asc, ascq })
    }
}

fn parse_range(expr: &ExprRange) -> CodeValue {
    let mut start_val: Option<u8> = None;
    let mut end_val: Option<u8> = None;

    let ExprRange {
        start, end, limits, ..
    } = expr;

    if let Some(start_expr) = start.as_deref() {
        if let Expr::Lit(ExprLit {
            lit: Lit::Int(start_lit),
            ..
        }) = start_expr
        {
            start_val = Some(start_lit.base10_parse::<u8>().unwrap());
        } else {
            panic!("range start must be an integer");
        }
    }

    if let Some(end_expr) = end.as_deref() {
        if let Expr::Lit(ExprLit {
            lit: Lit::Int(end_lit),
            ..
        }) = end_expr
        {
            end_val = Some(end_lit.base10_parse::<u8>().unwrap());
        } else {
            panic!("range end must be an integer");
        }
    }

    CodeValue::Range {
        start: start_val,
        end: end_val,
        limits: *limits,
    }
}

fn collect_or_chain(expr: &Expr, out: &mut Vec<u8>) {
    if let Expr::Binary(bin) = expr {
        collect_or_chain(&bin.left, out);
        collect_or_chain(&bin.right, out);
    } else if let Expr::Lit(ExprLit {
        lit: Lit::Int(lit), ..
    }) = expr
    {
        out.push(lit.base10_parse::<u8>().unwrap());
    } else {
        panic!("Invalid OR expression");
    }
}

fn parse_code_value(expr: &Expr) -> CodeValue {
    match expr {
        Expr::Infer(_) => CodeValue::WildCard,
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) => CodeValue::Exact(lit.base10_parse::<u8>().unwrap()),
        Expr::Binary(ExprBinary {
            op: BinOp::BitOr(_),
            ..
        }) => {
            let mut values = Vec::new();
            collect_or_chain(expr, &mut values);
            CodeValue::OneOf(values)
        }
        Expr::Range(range) => parse_range(range),
        _ => panic!("Invalid code value"),
    }
}

#[proc_macro_derive(MMCError, attributes(mmc_error))]
pub fn derive_mmc_error_enum(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let Data::Enum(data_enum) = data else {
        panic!("MMCError can only be derived for enums");
    };

    let mut from_code_arms = Vec::new();

    for variant in data_enum.variants {
        let ident = variant.ident;

        let attr = variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("mmc_error"))
            .expect("missing #[mmc_error(...)] attribute");

        let args: MacroArgs = attr.parse_args().expect("Invalid mmc_error args");

        let sk = args.sk.to_pattern();
        let asc = args.asc.to_pattern();
        let ascq = args.ascq.to_pattern();

        from_code_arms.push(quote! {
            (#sk, #asc, #ascq) => Some(Self::#ident)
        });
    }

    let expanded = quote! {
        impl #ident {
            pub fn from_codes(sk: u8, asc: u8, ascq: u8) -> Option<Self> {
                match (sk, asc, ascq) {
                    #(#from_code_arms,)*
                    _ => None,
                }
            }
        }
    };

    expanded.into()
}
