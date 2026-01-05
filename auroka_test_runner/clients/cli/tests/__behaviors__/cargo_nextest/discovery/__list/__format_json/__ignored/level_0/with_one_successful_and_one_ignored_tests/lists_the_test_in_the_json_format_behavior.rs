use crate::__steps__::Context;
use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use auroka_behavior::behavior;

behavior! {
    given_there_is_an_assembly_with(r#"
#[auroka_test]
fn pass() {
    assert_eq!(1, 1);
}

#[auroka_test]
#[ignore]
fn ignored() {
    assert_eq!(1, 1);
}
"#);
    when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments("--list --format json --ignored");

    "Outputs the test in the json format" {
        then_the_standard_output_should_have(r#"ignored: test"#);
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
