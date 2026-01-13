use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_macro_expansion_should_have;
use crate::__steps__::then_the_standard_error_should_not_have;
use crate::__steps__::when_the_macro_is_expanded;

const INPUT: &str = r#"
behavior! { :async
  given_there_is_something(Ordering::Equal)
  when_something_happens("HI")

  "Something is true" {
    then_something_should_be_true()
  }
}
"#;

const EXPECTED: &str = r#"
async fn something_is_true_inner() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context, Ordering::Equal).await?;
    when_something_happens(&mut context, "HI").await?;
    then_something_should_be_true(&context).await?;
    Ok(())
}
"#;

#[test]
pub fn expands_test_behavior() {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(&mut context, INPUT);
  when_the_macro_is_expanded(&mut context);
  then_the_standard_error_should_not_have(&context, "error:");
  then_the_macro_expansion_should_have(&context, EXPECTED);
}
