use super::element::Element;
use crate::backends::{Browser, Element as ElementBackend};
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

impl Browser for WebDriver {
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

  fn close(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    let driver = self.driver.clone();
    Box::pin(async move {
      driver.quit().await?;
      Ok(())
    })
  }
}
