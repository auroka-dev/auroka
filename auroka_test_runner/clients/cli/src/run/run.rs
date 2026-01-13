use crate::{Invocation, TestReport};
use auroka_test_registry::get_tests;
use clap::Parser;

/// Runs all registered tests.
/// This function is intended to be called from the main entry point of the test binary.
pub async fn run() -> TestReport {
  let _invocation = match Invocation::try_parse() {
    Ok(invocation) => invocation,
    Err(e) => e.exit(),
  };

  let tests = get_tests();
  let mut passed = 0;
  let mut failed = 0;

  println!("\nRunning auroka tests...\n");

  for test in tests {
    print!("test {} ... ", test.name);

    // In the future, we will capture panics here to report failures gracefully.
    // For now, a panic will abort the test runner, which is acceptable for step 1.
    match (test.test_fn)().await {
      Ok(_) => {
        println!("ok");
        passed += 1;
      }
      Err(e) => {
        println!("FAILED");
        println!("Error: {:?}", e);
        failed += 1;
      }
    }
  }

  println!("\ntest result: ok. {} passed; {} failed; 0 ignored; 0 measured\n", passed, failed);

  TestReport::new(passed, failed)
}
