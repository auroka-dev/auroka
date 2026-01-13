use super::Context;
use crate::expand::__helpers__::format_expansion_result;

pub fn when_the_macro_is_expanded(context: &mut Context) {
  let invocation_str = context
    .invocation()
    .as_ref()
    .expect("Invocation is not set");

  // The invocation string is `behavior! { ... }`.
  // We need to parse it as a macro to extract its content.
  let macro_call: syn::Macro = syn::parse_str(invocation_str).expect(&format!("Failed to parse macro invocation: {}", invocation_str));

  let input_tokens = macro_call.tokens;

  // Call the internal expansion logic
  let result = crate::expand(input_tokens);
  let (expansion, error) = format_expansion_result(result);

  context.expansion_set(expansion);
  context.error_set(error);
}
