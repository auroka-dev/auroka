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

#[derive(Clone, Debug)]
pub struct Geolocation {
  pub latitude: f64,
  pub longitude: f64,
  pub accuracy: Option<f64>,
}

#[derive(Clone, Debug)]
pub enum Viewport {
  /// 1280 x 720
  HD,
  /// 1920 x 1080
  FHD,
  /// 3840 x 2160
  UHD4K,
  /// 5120 x 2880
  UHD5K,
  /// Custom (width, height)
  Custom(u32, u32),
}

impl Viewport {
  pub fn dimensions(&self) -> (u32, u32) {
    match self {
      Viewport::HD => (1280, 720),
      Viewport::FHD => (1920, 1080),
      Viewport::UHD4K => (3840, 2160),
      Viewport::UHD5K => (5120, 2880),
      Viewport::Custom(w, h) => (*w, *h),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PageConfig {
  pub browser: Browser,
  pub viewport: Option<Viewport>,
  pub is_mobile: bool,
  pub is_landscape: bool,
  pub geolocation: Option<Geolocation>,
  pub permissions: Vec<String>,
}

impl Default for PageConfig {
  fn default() -> Self {
    Self {
      browser: Browser::Chromium,
      viewport: None,
      is_mobile: false,
      is_landscape: false,
      geolocation: None,
      permissions: Vec::new(),
    }
  }
}

impl From<Browser> for PageConfig {
  fn from(browser: Browser) -> Self {
    Self {
      browser,
      ..Default::default()
    }
  }
}

impl PageConfig {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn browser(mut self, browser: Browser) -> Self {
    self.browser = browser;
    self
  }

  pub fn viewport(mut self, width: u32, height: u32) -> Self {
    self.viewport = Some(Viewport::Custom(width, height));
    self
  }

  pub fn viewport_preset(mut self, preset: Viewport) -> Self {
    self.viewport = Some(preset);
    self
  }

  pub fn mobile(mut self, is_mobile: bool) -> Self {
    self.is_mobile = is_mobile;
    self
  }

  pub fn landscape(mut self, is_landscape: bool) -> Self {
    self.is_landscape = is_landscape;
    self
  }

  pub fn geolocation(mut self, latitude: f64, longitude: f64) -> Self {
    self.geolocation = Some(Geolocation {
      latitude,
      longitude,
      accuracy: None,
    });
    self
  }

  pub fn permissions(mut self, permissions: Vec<String>) -> Self {
    self.permissions = permissions;
    self
  }
}

#[derive(Clone)]
pub struct Page {
  backend: Arc<dyn Backend>,
  is_android: bool,
}

impl Page {
  pub async fn new() -> Result<Self> {
    Self::launch(PageConfig::default()).await
  }

  pub async fn launch<C>(config: C) -> Result<Self>
  where
    C: Into<PageConfig>,
  {
    let config = config.into();
    let is_android = matches!(
      config.browser,
      Browser::ChromeMobile | Browser::FirefoxMobile | Browser::OperaMobile
    );

    let backend: Arc<dyn Backend> = match config.browser {
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

    // Apply viewport configuration if backend supports it
    if let Some(viewport) = config.viewport {
      let (width, height) = viewport.dimensions();
      backend.set_viewport(width, height).await?;
    }

    Ok(Self {
      backend,
      is_android,
    })
  }

  pub async fn goto(url: &str) -> Result<Self> {
    let page = Self::new().await?;
    page.navigate(url).await?;
    Ok(page)
  }

  pub async fn navigate(&self, url: &str) -> Result<()> {
    let url = if self.is_android {
      url
        .replace("127.0.0.1", "10.0.2.2")
        .replace("localhost", "10.0.2.2")
    } else {
      url.to_string()
    };
    self.backend.goto(&url).await?;
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
