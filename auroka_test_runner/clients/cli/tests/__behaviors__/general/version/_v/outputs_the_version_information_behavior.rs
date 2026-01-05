use super::super::super::super::super::__steps__::Context;
use super::super::super::super::super::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_option;
use super::super::super::super::super::__steps__::standard_output::then_the_standard_output_should_have;
use super::super::super::super::super::__steps__::success::then_success_should_have_been_returned;
use auroka_behavior::behavior;

behavior! {
  when_auroka_test_runner_is_invoked_with_the_option("-V")

  "Outputs the auroka_test_runner version information" {
    then_the_standard_output_should_have(&format!("auroka_test_runner {}", env!("CARGO_PKG_VERSION")))
  }

  "Returns success" {
    then_success_should_have_been_returned()
  }
}
