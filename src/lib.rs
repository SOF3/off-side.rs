extern crate indent_stack;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use proc_macro::TokenStream as TokenStream1;

use proc_macro2::TokenStream as TokenStream2;

use crate::converter::Converter;

mod converter;

#[proc_macro]
pub fn off_side(input: TokenStream1) -> TokenStream1 {
    off_side_impl(input).unwrap_or_else(|err| TokenStream1::from(err.to_compile_error()))
}

fn off_side_impl(input: TokenStream1) -> syn::Result<TokenStream1> {
    let mut converter = Converter::default();
    converter.iterate(TokenStream2::from(input).into_iter())?;
    let stream = converter.collect()?;
    dbg!(&stream.to_string());
    Ok(stream.into())
}
