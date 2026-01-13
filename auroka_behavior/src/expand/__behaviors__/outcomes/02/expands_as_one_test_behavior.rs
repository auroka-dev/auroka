use super::super::super::super::__steps__::{Context, given_there_is_a_macro_invocation, then_the_macro_expansion_should_have, then_the_standard_error_should_not_have, when_the_macro_is_expanded};

const INPUT: &str = r#"
behavior! {
  given_there_is_something()
  when_something_happens()

  "Something is true" {
    then_something_should_be_true()
  }

  "Something else is true" {
    then_something_else_should_be_true()
  }
}"#;

const EXPECTED: &str = r#"
#[auroka::test]
fn compact() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context)?;
    when_something_happens(&mut context)?;
    let mut _errors_ = Vec::new();
    let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> anyhow::Result<()> {
            then_something_should_be_true(&context);
            Ok(())
        }),
    );
    match result {
        Ok(Err(e)) => _errors_.push(Box::new(e.to_string())),
        Err(payload) => _errors_.push(payload),
        Ok(Ok(())) => {}
    }
    let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> anyhow::Result<()> {
            then_something_else_should_be_true(&context);
            Ok(())
        }),
    );
    match result {
        Ok(Err(e)) => _errors_.push(Box::new(e.to_string())),
        Err(payload) => _errors_.push(payload),
        Ok(Ok(())) => {}
    }
    if !_errors_.is_empty() {
        std::panic::resume_unwind(_errors_.remove(0));
    }
    Ok(())
}
"#;

#[test]
pub fn expands_as_one_test_behavior() {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(&mut context, INPUT);
  when_the_macro_is_expanded(&mut context);
  then_the_standard_error_should_not_have(&context, "error:");
  then_the_macro_expansion_should_have(&context, EXPECTED);
}
