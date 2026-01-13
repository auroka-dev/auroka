mod behavior;
mod expand;
mod outcome;
use behavior::Behavior;
use expand::expand;
use outcome::Outcome;
use proc_macro::TokenStream;

/// Checks a behavior by running a setup sequence and verifying one or more outcomes.
///
/// # Example
///
/// ```rust
/// use auroka_behavior::behavior;
///
/// // Mock setup for the example
/// struct Context;
/// impl Context { fn new() -> Self { Self } }
/// fn given_some_state(ctx: &mut Context) -> anyhow::Result<()> { Ok(()) }
/// fn when_something_happens(ctx: &mut Context) -> anyhow::Result<()> { Ok(()) }
/// fn then_something_should_be_true(ctx: &Context) -> anyhow::Result<()> { Ok(()) }
///
/// behavior! {
///    given_some_state()
///    when_something_happens()
///
///    "The system works" {
///      then_something_should_be_true()
///    }
/// }
/// ```
#[proc_macro]
pub fn behavior(input: TokenStream) -> TokenStream {
  match expand(proc_macro2::TokenStream::from(input)) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}
