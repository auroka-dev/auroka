pub struct TestReport {
  passed: usize,
  failed: usize,
}

impl TestReport {
  pub fn new(passed: usize, failed: usize) -> Self {
    Self { passed, failed }
  }

  pub fn passed(&self) -> usize {
    self.passed
  }

  pub fn failed(&self) -> usize {
    self.failed
  }
}
