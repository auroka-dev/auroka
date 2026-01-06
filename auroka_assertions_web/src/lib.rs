use anyhow::Result;
use auroka_test_web::{Locator, expect};

pub async fn assert_has_text(locator: Locator, expected: &str) -> Result<()> {
  expect(locator).to_have_text(expected).await
}
