use super::element::Element;
use crate::backends::{Backend, Element as ElementBackend};
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use thirtyfour::prelude::{By, WebDriver as ThirtyFourWebDriver};
use thirtyfour::{Capabilities, ChromeCapabilities};

pub struct WebDriver {
  driver: ThirtyFourWebDriver,
}

impl WebDriver {
  pub async fn new(url: &str) -> Result<Self> {
    let caps = ChromeCapabilities::new();
    let driver = ThirtyFourWebDriver::new(url, caps).await?;
    Ok(Self { driver })
  }

  pub async fn with_caps<C>(url: &str, caps: C) -> Result<Self>
  where
    C: Into<Capabilities>,
  {
    let driver = ThirtyFourWebDriver::new(url, caps).await?;
    Ok(Self { driver })
  }
}

impl Backend for WebDriver {
  fn goto(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    let url = url.to_string();
    Box::pin(async move {
      self.driver.goto(&url).await?;
      Ok(())
    })
  }

  fn find_element(
    &self,
    selector: &str,
  ) -> Pin<Box<dyn Future<Output = Result<Box<dyn ElementBackend>>> + Send + '_>> {
    let selector = selector.to_string();
    Box::pin(async move {
      let element = self.driver.find(By::Css(&selector)).await?;
      Ok(Box::new(Element { element }) as Box<dyn ElementBackend>)
    })
  }

  fn content(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> {
    Box::pin(async move {
      let content = self.driver.source().await?;
      Ok(content)
    })
  }

  fn set_viewport(
    &self,
    width: u32,
    height: u32,
  ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    Box::pin(async move {
      // Set the window size. Usually this includes window decorations,
      // but it's the closest standard WebDriver command.
      self
        .driver
        .set_window_rect(0, 0, width as u32, height as u32)
        .await?;
      Ok(())
    })
  }

  fn close(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    let driver = self.driver.clone();
    Box::pin(async move {
      driver.quit().await?;
      Ok(())
    })
  }
}
