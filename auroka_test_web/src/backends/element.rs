use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub trait Element: Send + Sync {
  fn inner_text(&self) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + '_>>;
}
