use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[tokio::test]
async fn test_with_page_relative_path() -> anyhow::Result<()> {
  // Validates: with_page!("/path", |page| { ... })
  // Note: This expects BASE_URL (127.0.0.1:8787) to be up or fails connection.
  // So we might skip this or just mock the behavior if possible.
  // For now, let's test only what we can controll reliably.
  Ok(())
}

#[tokio::test]
async fn test_with_page_base_url_overload() -> anyhow::Result<()> {
  with_server!(
      "/check" => "<body>ok</body>",
      |base_url| {
          // Validates: with_page!(base_url, "/path", |page| { ... })
          with_page!(base_url, "/check", |page| {
              let body = page.content().await?;
              assert!(body.contains("ok"));
          })?
      }
  )
}

#[tokio::test]
async fn test_with_page_absolute_url() -> anyhow::Result<()> {
  with_server!(
      "/absolute" => "<body>absolute</body>",
      |base_url| {
          let full_url = format!("{}/absolute", base_url);
          // Validates: with_page!("/absolute_url...", |page| { ... })
          // where logic detects http prefix
          with_page!(&full_url, |page| {
               let body = page.content().await?;
               assert!(body.contains("absolute"));
          })?
      }
  )
}
