use crate::Page;
use std::future::Future;

const BASE_URL: &str = "http://127.0.0.1:8787";

pub async fn with_page<F, Fut>(path: &str, test_fn: F) -> anyhow::Result<()>
where
  F: FnOnce(Page) -> Fut,
  Fut: Future<Output = anyhow::Result<()>>,
{
  let url = format!("{}{}", BASE_URL, path);
  let page = Page::goto(&url).await?;
  let result = test_fn(page.clone()).await;
  let close_result = page.close().await;
  result.and(close_result)
}
