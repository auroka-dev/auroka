#![feature(assert_matches)]
mod expand;
use expand::expand;
use proc_macro::TokenStream;

/// A macro to transform a test function into a `tokio`-aware test with `inventory` registration.
///
/// This macro:
/// 1. Extracts the original function body into an inner function.
/// 2. Generates a wrapper `#[test]` function that sets up the `tokio` runtime (if async).
/// 3. Registers the test via `inventory` for discovery.
///
/// # Panics
///
/// This macro will emit a compile error (not panic) if the input is not a valid function.
#[proc_macro_attribute]
pub fn auroka_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
  expand(proc_macro2::TokenStream::from(item)).into()
}
