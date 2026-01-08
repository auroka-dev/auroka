use anyhow::{Context, Result};

#[doc(hidden)]
pub async fn assert_has_content_internal(url: &str, expected: &str) -> Result<()> {
  let response_text = reqwest::get(url)
    .await
    .context("Failed to fetch URL")?
    .text()
    .await
    .context("Failed to get response text")?;

  if response_text != expected {
    anyhow::bail!(
      "Content mismatch. Expected:\n{}\n\nActual:\n{}",
      expected,
      response_text
    );
  }
  Ok(())
}

#[macro_export]
macro_rules! assert_has_content {
  ($base:expr, $path:expr, $expected:expr) => {
    $crate::assert_has_content!(format!("{}{}", $base, $path), $expected)
  };
  ($url:expr, $expected:expr) => {
    $crate::assert_has_content::assert_has_content_internal(&$url, $expected).await?
  };
}
