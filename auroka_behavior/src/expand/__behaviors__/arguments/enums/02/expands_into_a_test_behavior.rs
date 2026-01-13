use super::super::super::super::super::__steps__::{Context, given_there_is_a_macro_invocation, then_the_macro_expansion_should_have, then_the_standard_error_should_not_have, when_the_macro_is_expanded};

const INPUT: &str = r#"
behavior! {
  given_there_is_something(Ordering::Equal, Ordering::Equal)
  when_something_happens(Ordering::Equal, Ordering::Equal)

  "Something is true" {
    then_something_should_be_true(Ordering::Equal, Ordering::Equal)
  }
}"#;

const EXPECTED: &str = r#"
#[auroka::test]
fn something_is_true() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context, Ordering::Equal, Ordering::Equal)?;
    when_something_happens(&mut context, Ordering::Equal, Ordering::Equal)?;
    then_something_should_be_true(&context, Ordering::Equal, Ordering::Equal);
    Ok(())
}
"#;

#[test]
pub fn expands_into_a_test_behavior() {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(&mut context, INPUT);
  when_the_macro_is_expanded(&mut context);
  then_the_standard_error_should_not_have(&context, "error:");
  then_the_macro_expansion_should_have(&context, EXPECTED);
}
