use super::super::super::super::super::__steps__::{Context, given_there_is_a_macro_invocation, then_the_macro_expansion_should_have, then_the_standard_error_should_not_have, when_the_macro_is_expanded};

#[test]
pub fn expands_as_one_test_behavior() {
  let mut context = Context::new();

  given_there_is_a_macro_invocation(
    &mut context,
    r#"
behavior! { :async
  given_there_is_something()
  when_something_happens()

  "Something is true" {
    then_something_should_be_true()
  }

  "Something else is true" {
    then_something_else_should_be_true()
  }
}"#,
  );

  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&context, "error:");

  then_the_macro_expansion_should_have(
    &context,
    r#"
#[auroka::test]
async fn compact() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context).await?;
    when_something_happens(&mut context).await?;
    let mut _errors_ = Vec::new();
    if let Err(err) = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            then_something_should_be_true(&context).await?;
        }),
    ) {
        _errors_.push(err);
    }
    if let Err(err) = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            then_something_else_should_be_true(&context).await?;
        }),
    ) {
        _errors_.push(err);
    }
    if !_errors_.is_empty() {
        std::panic::resume_unwind(_errors_.remove(0));
    }
    Ok(())
}
"#,
  );
}
