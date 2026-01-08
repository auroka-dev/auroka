use crate::backends::Element as ElementBackend;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub struct Element {
  pub(crate) element: chromiumoxide::Element,
}

impl ElementBackend for Element {
  fn inner_text(&self) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + '_>> {
    Box::pin(async move { Ok(self.element.inner_text().await?) })
  }
}
