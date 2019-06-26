// Copyright (C) 2019 chankyin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
    Ok(stream.into())
}
