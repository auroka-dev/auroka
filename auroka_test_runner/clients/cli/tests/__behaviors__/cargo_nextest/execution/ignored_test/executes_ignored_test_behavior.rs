use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::standard_output::then_the_standard_output_should_not_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::Context;
use crate::__steps__::TestMode;
use auroka_behavior::behavior;

behavior! {
  test_mode: TestMode

  given_there_is_an_assembly_with(r#"
#[test]
#[ignore]
fn ignored_one() {
    assert!(true);
}
"#);

  when_auroka_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(test_mode, "ignored_one --nocapture --exact --ignored");

  "Outputs its running 1 test" {
    then_the_standard_output_should_have("running 1 test");
  }

  "Outputs the successful test 1 summary" {
    then_the_standard_output_should_have("ignored_one ... ok");
  }

  "Outputs the successful test 1 standard output" {
    then_the_standard_output_should_have("test_one standard output");
  }

  "Outputs no error" {
    then_the_standard_error_should_be_empty();
  }

  "Outputs the assembly test summary" {
    then_the_standard_output_should_have("test result: ok. 1 passed; 0 failed; 0 ignored; 1 filtered out");
  }

  "Returns success" {
    then_success_should_have_been_returned();
  }
}
