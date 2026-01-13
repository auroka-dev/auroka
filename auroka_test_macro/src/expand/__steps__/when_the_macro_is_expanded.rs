use super::Context;

pub fn when_the_macro_is_expanded(context: &mut Context) {
  let invocation = context
    .invocation()
    .as_ref()
    .expect("Invocation is not set");

  // Parse the function from the string
  let mut item: syn::ItemFn = syn::parse_str(invocation).expect(&format!("Failed to parse function from invocation: {}", invocation));

  // Remove the `auroka_test` attribute because the proc_macro entry point receives the item without it.
  item
    .attrs
    .retain(|attr| !attr.path().is_ident("auroka_test"));

  // Convert back to TokenStream
  let input = quote::quote!(#item);

  // Call the internal expansion logic
  let output = crate::expand(input);

  let file: syn::File = syn::parse2(output).expect("Failed to parse expansion as File");
  let formatted = prettyplease::unparse(&file);

  // Store the result as string
  context.expansion_set(Some(formatted));
  context.error_set(Some(String::new()));
}
