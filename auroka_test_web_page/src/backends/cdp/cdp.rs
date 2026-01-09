use super::element::Element;
use crate::backends::{Backend, Element as ElementBackend};
use anyhow::Result;
use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use chromiumoxide::{Browser as CdpBrowser, BrowserConfig, Page as CdpPage};
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Mutex;

pub struct Cdp {
  page: Arc<CdpPage>,
  browser: Arc<Mutex<CdpBrowser>>,
  _temp_dir: TempDir,
}

impl Cdp {
  pub async fn new() -> Result<Self> {
    let temp_dir = tempfile::Builder::new()
      .prefix("auroka-test-cdp")
      .tempdir()?;

    let config = BrowserConfig::builder()
      .user_data_dir(temp_dir.path())
      .build()
      .map_err(|e| anyhow::anyhow!(e))?;

    let (browser, mut handler) = CdpBrowser::launch(config).await?;

    // Spawn the handler loop immediately so it can process messages
    tokio::spawn(async move {
      while let Some(h) = handler.next().await {
        if h.is_err() {
          break;
        }
      }
    });

    let page = Arc::new(browser.new_page("about:blank").await?);
    let browser = Arc::new(Mutex::new(browser));

    Ok(Self {
      page,
      browser,
      _temp_dir: temp_dir,
    })
  }
}

impl Backend for Cdp {
  fn goto(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    let url = url.to_string();
    Box::pin(async move {
      self.page.goto(&url).await?;
      Ok(())
    })
  }

  fn find_element(
    &self,
    selector: &str,
  ) -> Pin<Box<dyn Future<Output = Result<Box<dyn ElementBackend>>> + Send + '_>> {
    let selector = selector.to_string();
    Box::pin(async move {
      let element = self.page.find_element(&selector).await?;
      Ok(Box::new(Element { element }) as Box<dyn ElementBackend>)
    })
  }

  fn content(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> {
    Box::pin(async move {
      let content = self.page.content().await?;
      Ok(content)
    })
  }

  fn set_viewport(
    &self,
    width: u32,
    height: u32,
  ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    Box::pin(async move {
      let params = SetDeviceMetricsOverrideParams::builder()
        .width(width as i32)
        .height(height as i32)
        .device_scale_factor(1.0)
        .mobile(false)
        .build()
        // Unwrapping here because errors in building params are usually programmatic validation errors
        // and we want valid params. The builder might return Result<Params, String>.
        .map_err(|e| anyhow::anyhow!("Failed to build SetDeviceMetricsOverrideParams: {}", e))?;

      self.page.execute(params).await?;
      Ok(())
    })
  }

  fn close(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    Box::pin(async move {
      self.browser.lock().await.close().await?;
      Ok(())
    })
  }
}
