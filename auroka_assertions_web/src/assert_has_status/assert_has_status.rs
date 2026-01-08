use anyhow::{Context, Result};

#[doc(hidden)]
pub async fn assert_has_status_internal(url: &str, expected: u16) -> Result<()> {
  let response = reqwest::get(url).await.context("Failed to fetch URL")?;

  if response.status().as_u16() != expected {
    anyhow::bail!(
      "Status mismatch. Expected: {}, Actual: {}",
      expected,
      response.status().as_u16()
    );
  }
  Ok(())
}

#[macro_export]
macro_rules! assert_has_status {
  ($base:expr, $path:expr, $expected:expr) => {
    $crate::assert_has_status!(format!("{}{}", $base, $path), $expected)
  };
  ($url:expr, $expected:expr) => {
    $crate::assert_has_status::assert_has_status_internal(&$url, $expected).await?
  };
}
