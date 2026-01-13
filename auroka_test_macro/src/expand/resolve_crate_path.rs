use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;

pub(crate) fn resolve_crate_path() -> proc_macro2::TokenStream {
  if let Ok(found) = crate_name("auroka_test") {
    return match found {
      FoundCrate::Itself => quote! { crate },
      FoundCrate::Name(name) => {
        let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
        quote! { ::#ident }
      }
    };
  }

  if let Ok(found) = crate_name("auroka") {
    return match found {
      FoundCrate::Itself => quote! { crate::auroka_test },
      FoundCrate::Name(name) => {
        let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
        quote! { ::#ident::auroka_test }
      }
    };
  }

  quote! { ::auroka_test }
}
