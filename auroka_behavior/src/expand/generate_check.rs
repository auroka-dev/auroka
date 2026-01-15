use crate::Outcome;
use quote::quote;

pub(crate) fn generate_check(outcome: &Outcome, is_async: bool) -> proc_macro2::TokenStream {
  let block = &outcome.block();
  let stmts = &block.stmts;

  let execution = if is_async {
    quote! {
      let result = futures::FutureExt::catch_unwind(
        std::panic::AssertUnwindSafe(async {
          #( #stmts )*
          Ok::<(), anyhow::Error>(())
        })
      ).await;
    }
  } else {
    quote! {
      let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> anyhow::Result<()> {
          #( #stmts )*
          Ok(())
        }),
      );
    }
  };

  quote! {
    #execution

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
