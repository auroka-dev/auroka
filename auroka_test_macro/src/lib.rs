#![feature(assert_matches)]
mod expand;
use expand::expand;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn auroka_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
  expand(proc_macro2::TokenStream::from(item)).into()
}
