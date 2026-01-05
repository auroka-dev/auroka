use crate::__steps__::assembly::given_there_is_an_assembly_without_anything;
use crate::__steps__::standard_output::then_the_standard_output_should_be_empty;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_behavior::behavior;

behavior! {
    given_there_is_an_assembly_without_anything();
    when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments("--list --format terse --ignored");

    "Outputs nothing" {
        then_the_standard_output_should_be_empty();
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
