use super::Context;

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
  match crate::expand(input_tokens) {
    Ok(expanded_tokens) => {
      // Output tokens are a function definition (#[test] fn ...).
      // Format it nicely.
      // We parse as syn::File (which allows multiple items) to handle potential extra items or just the function.
      match syn::parse2::<syn::File>(expanded_tokens) {
        Ok(file) => {
          let formatted = prettyplease::unparse(&file);
          context.expansion_set(Some(formatted));
        }
        Err(e) => {
          // For now, if we can't parse as File, try to just to_string it for debugging
          context.error_set(Some(format!("Failed to parse output tokens: {}", e)));
        }
      }
    }
    Err(e) => {
      // This is a syn::Error from the macro expansion logic
      context.error_set(Some(e.to_string()));
    }
  }
}
