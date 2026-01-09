use crate::Browser;
use crate::Locator;
use crate::backends::Backend;
use crate::backends::Cdp;
use crate::backends::WebDriver;
use anyhow::Result;
use std::sync::Arc;
use thirtyfour::{FirefoxCapabilities, SafariCapabilities};

#[derive(Clone)]
pub struct Page {
  backend: Arc<dyn Backend>,
}

impl Page {
  pub async fn new() -> Result<Self> {
    Self::launch(Browser::Chromium).await
  }

  pub async fn launch(browser: Browser) -> Result<Self> {
    let backend: Arc<dyn Backend> = match browser {
      Browser::Chromium => Arc::new(Cdp::new().await?),
      Browser::Firefox => {
        let caps = FirefoxCapabilities::new();
        // Assuming geckodriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::Safari => {
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

  pub async fn content(&self) -> Result<String> {
    self.backend.content().await
  }

  pub async fn close(&self) -> Result<()> {
    self.backend.close().await?;
    Ok(())
  }
}
