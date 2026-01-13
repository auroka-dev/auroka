use super::{generate_function_name, generate_multi_test, generate_single_test, process_outcomes, process_setup_steps};
use crate::Behavior;

#[doc(hidden)]
pub fn expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
  let mut behavior = syn::parse2::<Behavior>(input)?;

  let setup_stmts = process_setup_steps(&mut behavior);
  process_outcomes(&mut behavior);

  let fn_name = generate_function_name(&behavior);

  if behavior.outcomes().len() == 1 {
    return Ok(generate_single_test(&behavior, &fn_name, &setup_stmts));
  }

  Ok(generate_multi_test(&behavior, &fn_name, &setup_stmts))
}
