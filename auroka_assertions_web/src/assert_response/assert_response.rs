use anyhow::{Context, Result};

#[doc(hidden)]
pub async fn assert_response_internal(
  url: &str,
  expected_status: u16,
  expected_content_type: &str,
  expected_body: &str,
) -> Result<()> {
  let response = reqwest::get(url).await.context("Failed to fetch URL")?;

  let status = response.status().as_u16();
  if status != expected_status {
    anyhow::bail!(
      "Status mismatch. Expected: {}, Actual: {}",
      expected_status,
      status
    );
  }

  let content_type = response
    .headers()
    .get(reqwest::header::CONTENT_TYPE)
    .and_then(|v| v.to_str().ok())
    .unwrap_or("");

  if !content_type.contains(expected_content_type) {
    anyhow::bail!(
      "Content-Type mismatch. Expected to contain: '{}', Actual: '{}'",
      expected_content_type,
      content_type
    );
  }

  let body = response
    .text()
    .await
    .context("Failed to get response text")?;
  if body != expected_body {
    anyhow::bail!(
      "Content mismatch. Expected:\n{}\n\nActual:\n{}",
      expected_body,
      body
    );
  }

  Ok(())
}

#[macro_export]
macro_rules! assert_response {
  ($base:expr, $path:expr, $status:expr, $content_type:expr, $body:expr) => {
    $crate::assert_response!(format!("{}{}", $base, $path), $status, $content_type, $body)
  };
  ($url:expr, $status:expr, $content_type:expr, $body:expr) => {
    $crate::assert_response::assert_response_internal(&$url, $status, $content_type, $body).await?
  };
}
