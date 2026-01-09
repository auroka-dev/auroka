use crate::Browser;
use crate::Locator;
use crate::backends::Backend;
use crate::backends::Cdp;
use crate::backends::WebDriver;
use anyhow::Result;
use std::sync::Arc;
use thirtyfour::{
  Capabilities, ChromeCapabilities, ChromiumLikeCapabilities, DesiredCapabilities,
  EdgeCapabilities, FirefoxCapabilities, SafariCapabilities,
};

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
      Browser::Chrome => {
        let caps = ChromeCapabilities::new();
        // Uses chromedriver. Defaults to looking for Google Chrome installation.
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::ChromeMobile => {
        let mut caps = ChromeCapabilities::new();
        // Connects to an Android Emulator or Device via ADB.
        // Requires 'com.android.chrome' installed on the device.
        caps.add_experimental_option("androidPackage", "com.android.chrome")?;
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::Firefox => {
        let caps = FirefoxCapabilities::new();
        // Assuming geckodriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::FirefoxMobile => {
        let caps = FirefoxCapabilities::new();
        let mut caps: Capabilities = caps.into();
        // Connects to Firefox on Android via GeckoView
        caps.insert(
          "moz:firefoxOptions".to_string(),
          serde_json::json!({
            "androidPackage": "org.mozilla.firefox"
          }),
        );
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::Safari => {
        let caps = SafariCapabilities::new();
        // Assuming safaridriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::SafariTechnologyPreview => {
        let caps = SafariCapabilities::new();
        let mut caps: Capabilities = caps.into();
        // Targeting Safari Technology Preview
        caps.insert(
          "safari:technologyPreview".to_string(),
          serde_json::json!(true),
        );
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::SafariMobile => {
        let caps = SafariCapabilities::new();
        let mut caps: Capabilities = caps.into();
        // Targeting iOS Simulator via safaridriver.
        // Note: The user might need to run `safaridriver --enable` and specifically ensure
        // they are running a driver instance that can see the simulator (often just the system one).
        caps.insert("platformName".to_string(), serde_json::json!("iOS"));
        caps.insert(
          "deviceName".to_string(),
          serde_json::json!("iPhone Simulator"),
        );
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::Edge => {
        let caps = EdgeCapabilities::new();
        // Assuming msedgedriver is running on port 4444
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::Opera => {
        // Opera is Chromium-based, traditionally uses "opera" as browser name.
        // Requires operadriver.
        let caps = DesiredCapabilities::opera();
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::OperaMobile => {
        // Opera Mobile on Android
        let caps = DesiredCapabilities::opera();
        let mut caps: Capabilities = caps.into();
        // Manually adding androidPackage for Opera which is Chromium based but we have raw capabilities here.
        caps.insert(
          "goog:chromeOptions".to_string(),
          serde_json::json!({ "androidPackage": "com.opera.browser" }),
        );
        Arc::new(WebDriver::with_caps("http://localhost:4444", caps).await?)
      }
      Browser::WebKit => {
        // Targeting generic WebKit (e.g. GNOME Web / Epiphany via generic WebDriver or Playwright's WebKit).
        // Standard "browserName" capability is "webkit" or sometimes "epiphany".
        // We default to "webkit" which is the standard identifier.
        // Requires a WebDriver compatible with WebKit (e.g. WebKitWebDriver) running.
        let mut caps = Capabilities::new();
        caps.insert("browserName".to_string(), serde_json::json!("webkit"));
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
