use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_macro_expansion_should_have;
use crate::__steps__::then_the_standard_error_should_not_have;
use crate::__steps__::when_the_macro_is_expanded;

#[test]
pub fn expands_worker_test_behavior() {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(
    &mut context,
    r#"
#[auroka_test]
async fn my_async_test() {
  let _ = 1 + 1;
}
"#,
  );
  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&mut context, "error:");

  then_the_macro_expansion_should_have(
    &mut context,
    r#"
async fn my_async_test_inner() {
    let _ = 1 + 1;
}
"#,
  );

  then_the_macro_expansion_should_have(
    &mut context,
    r#"
fn my_async_test() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(my_async_test_inner())
}
"#,
  );
}
