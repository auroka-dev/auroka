use anyhow::{Context, Result};

#[doc(hidden)]
pub async fn assert_has_content_type_internal(url: &str, expected: &str) -> Result<()> {
  let response = reqwest::get(url).await.context("Failed to fetch URL")?;

  let content_type = response
    .headers()
    .get(reqwest::header::CONTENT_TYPE)
    .and_then(|v| v.to_str().ok())
    .unwrap_or("");

  if !content_type.contains(expected) {
    anyhow::bail!("Content-Type mismatch. Expected to contain: '{}', Actual: '{}'", expected, content_type);
  }
  Ok(())
}

#[macro_export]
macro_rules! assert_has_content_type {
  ($base:expr, $path:expr, $expected:expr) => {
    $crate::assert_has_content_type!(format!("{}{}", $base, $path), $expected)
  };
  ($url:expr, $expected:expr) => {
    $crate::assert_has_content_type::assert_has_content_type_internal(&$url, $expected).await?
  };
}
