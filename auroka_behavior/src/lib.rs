mod behavior;
mod expand;
mod outcome;
use behavior::Behavior;
use expand::expand;
use outcome::Outcome;
use proc_macro::TokenStream;

#[proc_macro]
pub fn behavior(input: TokenStream) -> TokenStream {
  match expand(proc_macro2::TokenStream::from(input)) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}
