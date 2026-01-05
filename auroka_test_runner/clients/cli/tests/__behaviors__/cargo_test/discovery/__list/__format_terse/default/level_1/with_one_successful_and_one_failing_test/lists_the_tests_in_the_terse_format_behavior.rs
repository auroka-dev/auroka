use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_behavior::behavior;

behavior! {
    given_there_is_an_assembly_with(r#"
mod level_1 {
    use auroka_test::*;

    #[auroka_test]
    fn pass() {
        assert_eq!(1, 1);
    }

    #[auroka_test]
    fn fail() {
        assert_eq!(1, 2);
    }
}
"#);
    when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments("--list --format terse");

    "Outputs the test in the terse format" {
        then_the_standard_output_should_have(r#"level_1::pass: test
level_1::fail: test"#);
   }

   "Returns success" {
        then_success_should_have_been_returned();
   }
}
