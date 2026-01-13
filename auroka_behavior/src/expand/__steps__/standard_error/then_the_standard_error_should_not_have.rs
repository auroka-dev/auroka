use super::super::Context;

pub fn then_the_standard_error_should_not_have(context: &Context, content: &str) {
  if let Some(obtained) = context.error() {
    assert!(!obtained.contains(content), "Expected Standard Error to not contain '{}' but its {}", content, obtained);
  }
}
