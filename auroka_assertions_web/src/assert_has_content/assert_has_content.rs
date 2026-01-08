use anyhow::{Context, Result};
use reqwest::{Client, Method};
use std::str::FromStr;

#[doc(hidden)]
pub async fn assert_has_content_internal(
  url: &str,
  method: Option<&str>,
  expected: &str,
) -> Result<()> {
  let method_str = method.unwrap_or("GET").to_uppercase();
  let method = Method::from_str(&method_str).context("Invalid HTTP method")?;

  let client = Client::new();
  let response_text = client
    .request(method, url)
    .send()
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
  ($base:expr, $path:expr, : $method:ident, $expected:expr) => {
    $crate::assert_has_content::assert_has_content_internal(
      &format!("{}{}", $base, $path),
      Some(stringify!($method)),
      $expected,
    )
    .await?
  };
  ($base:expr, $path:expr, $expected:expr) => {
    $crate::assert_has_content::assert_has_content_internal(
      &format!("{}{}", $base, $path),
      None,
      $expected,
    )
    .await?
  };
  ($url:expr, $expected:expr) => {
    $crate::assert_has_content::assert_has_content_internal(&$url, None, $expected).await?
  };
}
