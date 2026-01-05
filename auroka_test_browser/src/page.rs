use crate::Locator;
use crate::backends::Browser;
use crate::backends::Chromium;
use crate::backends::WebDriver;
use anyhow::Result;
use std::sync::Arc;
use thirtyfour::{FirefoxCapabilities, SafariCapabilities};

#[derive(Clone)]
pub struct Page {
  backend: Arc<dyn Browser>,
}

pub enum BrowserType {
  Chromium,
  Firefox,
  Safari,
}

impl Page {
  pub async fn new() -> Result<Self> {
    Self::launch(BrowserType::Chromium).await
  }

  pub async fn launch(browser_type: BrowserType) -> Result<Self> {
    let backend: Arc<dyn Browser> = match browser_type {
      BrowserType::Chromium => Arc::new(Chromium::new().await?),
      BrowserType::Firefox => {
        let caps = FirefoxCapabilities::new();
        // Assuming geckodriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      BrowserType::Safari => {
        let caps = SafariCapabilities::new();
        // Assuming safaridriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
    };
    Ok(Self { backend })
  }

  pub async fn goto(url: &str) -> Result<Self> {
    let page = Self::new().await?;
    page.backend.goto(url).await?;
    Ok(page)
  }

  pub async fn navigate(&self, url: &str) -> Result<()> {
    self.backend.goto(url).await?;
    Ok(())
  }

  pub fn locator(&self, selector: &str) -> Locator {
    Locator::new(self.backend.clone(), selector)
  }

  pub async fn close(&self) -> Result<()> {
    self.backend.close().await?;
    Ok(())
  }
}
