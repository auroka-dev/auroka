use super::Context;
use auroka_assertions::assert_contains_diff;

pub fn then_the_macro_expansion_should_have(context: &Context, expansion: &str) {
  let obtained = context
    .expansion()
    .as_ref()
    .expect("Expected macro expansion to be set");

  assert_contains_diff!(obtained, expansion.trim(), "Expected macro expansion to contain '{}'", expansion);
}
