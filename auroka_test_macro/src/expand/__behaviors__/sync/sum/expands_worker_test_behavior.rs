use super::super::super::super::__steps__::{Context, given_there_is_a_macro_invocation, then_the_macro_expansion_should_have, then_the_standard_error_should_not_have, when_the_macro_is_expanded};

const INVOCATION: &str = r#"
#[auroka_test]
fn my_test() {
  let _ = 1 + 1;
}
"#;

#[test]
pub fn expands_worker_test_behavior() {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(&mut context, INVOCATION);
  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&mut context, "error:");

  then_the_macro_expansion_should_have(
    &mut context,
    r#"fn my_test_inner() {
    let _ = 1 + 1;
}"#,
  );

  then_the_macro_expansion_should_have(
    &mut context,
    r#"#[test]
fn my_test() {
    my_test_inner()
}"#,
  );
}
