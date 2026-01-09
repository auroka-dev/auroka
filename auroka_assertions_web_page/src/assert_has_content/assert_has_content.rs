use anyhow::{Result, anyhow};
use auroka_test_web_page::Page;

#[doc(hidden)]
pub async fn assert_has_content_internal(page: &Page, expected: &str) -> Result<()> {
  let content = page.content().await?;
  if content.contains(expected) {
    Ok(())
  } else {
    Err(anyhow!(
      "Expected page content to contain '{}', but it did not.",
      expected
    ))
  }
}

#[macro_export]
macro_rules! assert_has_content {
  ($page:expr, $expected:expr) => {
    $crate::assert_has_content_internal($page, $expected).await?
  };
}
