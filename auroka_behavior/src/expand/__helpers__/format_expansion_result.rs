use proc_macro2::TokenStream;

pub fn format_expansion_result(result: syn::Result<TokenStream>) -> (Option<String>, Option<String>) {
  match result {
    Ok(expanded_tokens) => {
      // Output tokens are a function definition (#[test] fn ...).
      // Format it nicely.
      // We parse as syn::File (which allows multiple items) to handle potential extra items or just the function.
      match syn::parse2::<syn::File>(expanded_tokens) {
        Ok(file) => (Some(prettyplease::unparse(&file)), None),
        Err(e) => (None, Some(format!("Failed to parse output tokens: {}", e))),
      }
    }
    Err(e) => (None, Some(e.to_string())),
  }
}
