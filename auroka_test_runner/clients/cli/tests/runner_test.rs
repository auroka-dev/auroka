use auroka_test::auroka_test;

#[auroka_test]
fn test_a() {
  assert!(true);
}

#[auroka_test]
async fn test_b() {
  assert!(true);
}

// This is a "meta-test": it runs the custom runner, which runs the tests above.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn test_custom_runner_execution() {
  let report = auroka_test_runner::run().await;
  assert_eq!(report.passed(), 2);
  assert_eq!(report.failed(), 0);
}
