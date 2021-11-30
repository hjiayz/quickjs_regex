use proc_macro::TokenStream;
use proc_macro2::Span;
use quickjs_regex_backend::*;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Ident, LitStr};

#[proc_macro]
pub fn uregex(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as LitStr);
    gen(&s.value(), UNICODE)
}

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let params: Punctuated<LitStr, Comma> =
        parse_macro_input!(input with Punctuated::parse_terminated);
    let mut iter = params.iter();
    let regex = iter.next().expect("missing regexp.").value();
    let param = iter.next().map(|param| param.value()).unwrap_or_default();
    let mut flag = NAMED_GROUPS;
    for ch in param.chars().next() {
        match ch {
            'y' => flag = flag | STICKY,
            'g' => flag = flag | GLOBAL,
            'u' => flag = flag | UNICODE,
            'i' => flag = flag | IGNORECASE,
            'm' => flag = flag | MULTILINE,
            _ => (),
        }
    }
    gen(&regex, flag)
}

fn gen(regex: &str, flag: Flag) -> TokenStream {
    use proc_macro_crate::*;
    let found_crate = crate_name("quickjs_regex").unwrap();
    let crate_name = match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    };
    let regex = Regex::compile(regex, flag).unwrap();
    let byte_code = regex.byte_code();

    let capture_count = regex.capture_count();
    let expanded = quote! {
        {
            use #crate_name::Regex;
            Regex::from_static(&[#(#byte_code),*],#capture_count)
        }
    };
    TokenStream::from(expanded)
}
