use proc_macro2::TokenStream;

pub fn format_expansion_result(result: syn::Result<TokenStream>) -> (Option<String>, Option<String>) {
  match result {
    Ok(expanded_tokens) => {
      // Use rustfmt to respect project configuration (e.g. 2 spaces)
      // We accept that if rustfmt fails, the raw string uses default spacing which might break tests
      (Some(auroka_utils::format_code(&expanded_tokens.to_string())), None)
    }
    Err(e) => (None, Some(e.to_string())),
  }
}
