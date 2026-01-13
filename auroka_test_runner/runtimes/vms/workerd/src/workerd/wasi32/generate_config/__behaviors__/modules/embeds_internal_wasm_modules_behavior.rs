use super::super::super::__steps__::Context;
use super::super::super::__steps__::given_the_assembly_path_is;
use super::super::super::__steps__::then_the_config_should_contain;
use super::super::super::__steps__::when_the_config_is_generated;

#[test]
pub fn embeds_internal_wasm_modules_behavior() {
  let mut context = Context::new();

  given_the_assembly_path_is(&mut context, "/tmp/test_worker.wasm");

  when_the_config_is_generated(&mut context);

  then_the_config_should_contain(&mut context, r#"(name = "/tmp/test_worker.internal.0.wasm", wasm = embed "/tmp/test_worker.internal.0.wasm")"#);

  then_the_config_should_contain(&mut context, r#"(name = "/tmp/test_worker.internal.1.wasm", wasm = embed "/tmp/test_worker.internal.1.wasm")"#);
}
