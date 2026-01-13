use crate::Outcome;
use quote::quote;

pub(crate) fn generate_check(outcome: &Outcome) -> proc_macro2::TokenStream {
  let block = &outcome.block();
  let stmts = &block.stmts;
  quote! {
      if let Err(err) = std::panic::catch_unwind(
          std::panic::AssertUnwindSafe(|| {
              #( #stmts )*
          }),
      ) {
          _errors_.push(err);
      }
  }
}
