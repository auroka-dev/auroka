use super::{generate_function_name, generate_multi_test, generate_single_test, process_outcomes, process_setup_steps};
use crate::Behavior;

#[doc(hidden)]
pub fn expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
  let mut behavior = syn::parse2::<Behavior>(input)?;
  let is_async = behavior.is_async();

  let setup_stmts = process_setup_steps(is_async, behavior.setup_steps_mut());
  process_outcomes(is_async, behavior.outcomes_mut());

  let fn_name = generate_function_name(behavior.outcomes());

  if behavior.outcomes().len() == 1 { Ok(generate_single_test(&behavior, &fn_name, &setup_stmts)) } else { Ok(generate_multi_test(&behavior, &fn_name, &setup_stmts)) }
}
