use crate::Outcome;
use quote::quote;

pub(crate) fn generate_check(outcome: &Outcome) -> proc_macro2::TokenStream {
  let block = &outcome.block();
  let stmts = &block.stmts;
  quote! {
      let result = std::panic::catch_unwind(
          std::panic::AssertUnwindSafe(|| -> anyhow::Result<()> {
              #( #stmts )*
              Ok(())
          }),
      );

      match result {
        Ok(Err(e)) => _errors_.push(Box::new(e.to_string())),
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            _errors_.push(Box::new(msg));
        },
        Ok(Ok(())) => {}
      }
  }
}
