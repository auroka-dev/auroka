use crate::__steps__::Context;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_option;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use auroka_behavior::behavior;

behavior! {
  when_auroka_test_runner_is_invoked_with_the_option("--help")

  "Outputs the auroka-test-runner help information" {
    then_the_standard_output_should_have(
      r#"Execute all unit and integration tests and build examples of a local package

Usage: auroka_test_runner [OPTIONS] [INPUT] [FILTER]

Arguments:
  [INPUT]   The wasm file to test
  [FILTER]  The filter string to match test names against

Options:
      --include-ignored  Include ignored tests in the test run
      --ignored          Run only ignored tests
      --nocapture        Do not capture stdout/stderr
      --list             List all tests that would be run
      --exact            Exactly match filters rather than by substring
      --skip <PATTERN>   Skip tests whose names match the given pattern
      --format <FORMAT>  Configure formatting of output [possible values: pretty, terse, json]
  -h, --help             Print help
  -V, --version          Print version
"#,
    )
  }

  "Returns success" {
    then_success_should_have_been_returned()
  }
}
