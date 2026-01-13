use super::super::super::super::__steps__::{
  cargo_nextest::Context,
  package::given_there_is_a_package_with_100_successful_tests,
  standard_error::{then_the_standard_error_should_have, then_the_standard_error_should_not_have},
  success::then_success_should_have_been_returned,
  when_cargo_test_is_invoked_over_the_package,
};
use workers_behavior::behavior;

behavior! {
    given_there_is_a_package_with_100_successful_tests();
    when_cargo_test_is_invoked_over_the_package();

    "Outputs its running 100 tests" {
      then_the_standard_error_should_have("Starting 100 tests across 1 binary");
    }

    "Outputs the tests execution summary" {
      then_the_standard_error_should_have("100 tests run: 100 passed, 0 skipped");
    }

    "Outputs no error" {
      then_the_standard_error_should_not_have("error:");
    }

    "Returns success" {
      then_success_should_have_been_returned();
    }
}
