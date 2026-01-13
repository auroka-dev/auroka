use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;

pub(crate) fn resolve_crate_path() -> proc_macro2::TokenStream {
  match crate_name("auroka_test") {
    Ok(FoundCrate::Itself) => quote! { crate },
    Ok(FoundCrate::Name(name)) => {
      let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
      quote! { ::#ident }
    }
    Err(_) => {
      // If auroka_test is not found directly, try auroka
      match crate_name("auroka") {
        Ok(FoundCrate::Itself) => quote! { crate::auroka_test },
        Ok(FoundCrate::Name(name)) => {
          let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
          quote! { ::#ident::auroka_test }
        }
        Err(_) => {
          // Fallback to assuming auroka_test is in scope
          quote! { ::auroka_test }
        }
      }
    }
  }
}
