use crate::__steps__::Context;
use crate::__steps__::TestMode;
use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::auroka_test_runner::when_auroka_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments;
use crate::__steps__::failure::then_failure_should_have_been_returned;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use auroka_behavior::behavior;

behavior! {
  test_mode: TestMode

  given_there_is_a_javascript_bundle_with(r#"
    exports.test_failure = function() {
        throw new Error("This error should map back to the original source");
    };
    // This is a base64 encoded source map that maps the code above to 'original_source.ts'
    //# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiYnVuZGxlLmpzIiwic291cmNlcyI6WyJvcmlnaW5hbF9zb3VyY2UudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUNBLHdCQUF3QixvQkFBb0IiLCJzb3VyY2VzQ29udGVudCI6WyJcbmV4cG9ydCBmdW5jdGlvbiB0ZXN0X2ZhaWx1cmUoKSB7XG4gICAgdGhyb3cgbmV3IEVycm9yKFwiVGhpcyBlcnJvciBzaG91bGQgbWFwIGJhY2sgdG8gdGhlIG9yaWdpbmFsIHNvdXJjZVwiKTtcbn0iXX0=
  "#);

  when_auroka_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(test_mode, "");

  "Outputs the failure message" {
    then_the_standard_output_should_have("This error should map back to the original source");
  }

  "Outputs the original source filename in the stack trace" {
    then_the_standard_output_should_have("original_source.ts");
  }

  "Returns failure" {
    then_failure_should_have_been_returned();
  }
}
