use super::{generate_function_name, generate_multi_test, generate_single_test, process_outcomes, process_setup_steps};
use crate::Behavior;

#[doc(hidden)]
pub fn expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
  let mut input = syn::parse2::<Behavior>(input)?;
  let is_async = input.is_async();

  let setup_stmts = process_setup_steps(is_async, input.setup_steps_mut());
  process_outcomes(is_async, input.outcomes_mut());

  let outcomes = input.outcomes();
  let fn_name = generate_function_name(outcomes);

  if outcomes.len() == 1 { Ok(generate_single_test(is_async, &fn_name, &setup_stmts, outcomes)) } else { Ok(generate_multi_test(is_async, &fn_name, &setup_stmts, outcomes)) }
}
