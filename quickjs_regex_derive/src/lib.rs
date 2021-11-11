use proc_macro::TokenStream;
use quickjs_regex::{Regex, UNICODE};
use quote::quote;
use syn::token::Bracket;
use syn::{Expr, ExprArray, ExprLit};
use syn::__private::Span;
use syn::{punctuated::Punctuated, parse_macro_input, token::Comma,Lit, LitInt, LitStr};

#[proc_macro]
pub fn uregex(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as LitStr);
    let regex = Regex::compile(&s.value(), UNICODE).unwrap();

    let elems : Punctuated<Expr, Comma> = regex.byte_code().into_iter().map(|byte|{
        Expr::Lit(ExprLit{attrs:Default::default(),lit:Lit::Int(LitInt::new(&byte.to_string(),Span::call_site()))})
    }).collect();

    let byte_code = ExprArray{
        attrs:vec![],
        bracket_token:Bracket::default(),
        elems:elems,
    };

    let capture_count = LitInt::new(&format!("{}", regex.capture_count()), Span::call_site());
    let expanded = quote! {
        {
            use quickjs_regex::Regex;
            Regex::from_static(&#byte_code,#capture_count)
        }
    };
    TokenStream::from(expanded)
}
