use crate::backends::Element as ElementBackend;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use thirtyfour::prelude::*;

pub struct Element {
  pub(crate) element: WebElement,
}

impl ElementBackend for Element {
  fn inner_text(&self) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + '_>> {
    Box::pin(async move { Ok(Some(self.element.text().await?)) })
  }
}
