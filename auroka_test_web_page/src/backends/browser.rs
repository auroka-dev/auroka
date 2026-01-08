use crate::backends::Element;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub trait Browser: Send + Sync {
  fn goto(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
  fn find_element(
    &self,
    selector: &str,
  ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Element>>> + Send + '_>>;
  fn content(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>>;
  fn close(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}
