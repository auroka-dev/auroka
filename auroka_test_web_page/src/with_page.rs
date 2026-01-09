use crate::Page;
use std::future::Future;

#[doc(hidden)]
pub async fn with_page_internal<F, Fut>(path: &str, test_fn: F) -> anyhow::Result<()>
where
  F: FnOnce(Page) -> Fut,
  Fut: Future<Output = anyhow::Result<()>>,
{
  if !path.starts_with("http") {
    return Err(anyhow::anyhow!(
      "URL must start with 'http' or 'https'. Relative paths matching implicit base URL are no longer supported."
    ));
  }
  let url = path;
  let page = Page::goto(url).await?;
  let result = test_fn(page.clone()).await;
  let close_result = page.close().await;
  result.and(close_result)
}

#[macro_export]
macro_rules! with_page {
  ($base:expr, $path:expr, |$page:ident| $body:block) => {
    $crate::with_page!(&format!("{}{}", $base, $path), |$page| $body)
  };
  ($path:expr, |$page:ident| $body:block) => {
    $crate::with_page_internal($path, |$page| async move {
      $body
      Ok(())
    })
    .await
  };
}
