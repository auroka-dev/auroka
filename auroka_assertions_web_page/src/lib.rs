use anyhow::Result;
use auroka_test_web_page::{Locator, expect};

#[doc(hidden)]
pub async fn assert_has_text_internal(locator: Locator, expected: &str) -> Result<()> {
  expect(locator).to_have_text(expected).await
}

#[macro_export]
macro_rules! assert_has_text {
  ($locator:expr, $expected:expr) => {
    $crate::assert_has_text_internal($locator, $expected).await?
  };
}
