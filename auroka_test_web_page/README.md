# auroka_test_web_page

A high-level, Playwright-inspired browser automation library for `auroka` tests.

This crate provides a clean, async API to control browsers (Chromium, Firefox, Safari) and assert on page content. It is designed to be used in conjunction with `auroka_test` and `auroka_test_web`.

## Features

-   **Multi-Backend Support**:
    -   **Chromium (CDP)**: Direct integration via Chrome DevTools Protocol for fast, reliable automation.
    -   **WebDriver**: Support for Firefox and Safari via the `thirtyfour` crate (requires `geckodriver` or `safaridriver`).
-   **Modern API**:
    -   **`Page`**: Represents a browser tab.
    -   **`Locator`**: Encapsulates a selector to query elements.
    -   **`expect(...)`**: Fluent assertion API.
-   **Macros**:
    -   `with_page!`: Helper to manage browser lifecycle and navigation.

## Browser Setup & Dependencies

By default, `auroka_test_web_page` uses **Chromium** via the Chrome DevTools Protocol (CDP). Explicitly installing drivers is only necessary if you plan to use other browsers (Firefox, Safari, etc.).

### Supported Browsers

| Browser                 | Variant Enum                       | Backend   | Driver Requirement            |
| :---------------------- | :--------------------------------- | :-------- | :---------------------------- |
| **Chromium** (Default)  | `Browser::Chromium`                | CDP       | Chrome or Chromium installed. |
| **Google Chrome**       | `Browser::Chrome`                  | CDP       | Chrome installed.             |
| **Firefox**             | `Browser::Firefox`                 | WebDriver | `geckodriver`                 |
| **Safari** (macOS)      | `Browser::Safari`                  | WebDriver | `safaridriver` (Built-in)     |
| **Safari Tech Preview** | `Browser::SafariTechnologyPreview` | WebDriver | `safaridriver`                |
| **Microsoft Edge**      | `Browser::Edge`                    | CDP       | Edge installed.               |
| **Opera**               | `Browser::Opera`                   | CDP       | Opera installed.              |
| **WebKit** (GNOME Web)  | `Browser::WebKit`                  | WebDriver | `webkit-webdriver`            |

### Mobile Support

| Platform                     | Variant Enum                     | Backend   | Requirements                                               |
| :--------------------------- | :------------------------------- | :-------- | :--------------------------------------------------------- |
| **Chrome Mobile (Emulated)** | `Browser::ChromeMobileEmulation` | CDP       | Chrome installed. Emulates mobile on Desktop (No ADB/SDK). |
| **Edge Mobile (Emulated)**   | `Browser::EdgeMobileEmulation`   | CDP       | Edge installed. Emulates mobile on Desktop (No ADB/SDK).   |
| **Opera Mobile (Emulated)**  | `Browser::OperaMobileEmulation`  | CDP       | Opera installed. Emulates mobile on Desktop (No ADB/SDK).  |
| **Android (Chrome)**         | `Browser::ChromeMobile`          | WebDriver | Android SDK, `chromedriver`, Emulator/Device with Chrome.  |
| **Android (Edge)**           | `Browser::EdgeMobile`            | WebDriver | Android SDK, `msedgedriver`, Emulator/Device with Edge.    |
| **Android (Firefox)**        | `Browser::FirefoxMobile`         | WebDriver | Android SDK, `geckodriver`, Emulator/Device with Firefox.  |
| **Android (Opera)**          | `Browser::OperaMobile`           | WebDriver | Android SDK, `operadriver`, Emulator/Device with Opera.    |
| **iOS (Safari)**             | `Browser::SafariMobile`          | WebDriver | Xcode, iOS Simulator.                                      |

### Installation (macOS via Homebrew)

```bash
# Browsers
brew install --cask google-chrome firefox microsoft-edge opera

# Drivers
brew install chromedriver geckodriver msedgedriver operadriver
```

**Safari**: Run `safaridriver --enable`.

**Android**: Install Android Studio, ensure `adb` is in PATH, and start an emulator.

## Usage

### Basic Navigation (Default Chromium)

```rust
use auroka_test_web_page::{with_page, expect};

#[auroka::test]
async fn test_example_com() -> anyhow::Result<()> {
    // Pass a full URL to navigate explicitly
  with_page! {
    "https://example.com", |page| {
      let content = page.content().await?;
      assert!(content.contains("Example Domain"));
    }
  }
}
```

### Using Specific Browsers

To use a specific browser (e.g., Firefox, Edge), simply use the browser tag in the `with_page!` macro.

```rust
use auroka_test_web_page::with_page;
use auroka_assertions_web_page::assert_has_text;

#[auroka::test]
async fn test_on_firefox() -> anyhow::Result<()> {
  with_page! { :Firefox
    "https://example.com", |page| {
          assert_has_text!(page.locator("h1"), "Example Domain");
    }
  }
}
```

### Viewport and Device Emulation

You can configure the viewport size, orientation, and mobile emulation mode declaratively.

```rust
use auroka_test_web_page::with_page;

#[auroka::test]
async fn test_mobile_layout() -> anyhow::Result<()> {
  with_page! { :Chrome :Mobile :Landscape :FHD
    "https://example.com", |page| {
        // Test in a simulated FHD Mobile Landscape environment
    }
  }
}
```

Supported flags:
-   **Browsers**: `:Chrome`, `:Chromium`, `:Firefox`, `:Safari`, `:Edge`, etc.
-   **Presets**: `:HD` (1280x720), `:FHD` (1920x1080), `:4K`, `:5K`.
-   **Modes**: `:Mobile`, `:Landscape`.
-   **Custom**: `:1024 x 768`.
-   **Advanced**: `:Permissions(["camera"])`, `:GeoLocation(37.7, -122.4)`.

### Locators and Declarative Assertions

```rust
use auroka_test::web::page::{assert_has_text, with_page};

#[auroka::test]
async fn test_element_text() -> anyhow::Result<()> {
  with_page! {
    "https://example.com", |page| {
      // Find element and assert text content using macro
      assert_has_text!(page.locator("h1"), "Example Domain");
    }
  }
}
```

### Integration with `with_server!`

When testing local handlers, combine `with_page!` with `with_server!` to get an ephemeral test server and base URL.

```rust
use auroka_test::web::page::{assert_has_text, with_page};
use auroka_test::web::with_server;

#[auroka::test]
async fn test_local_handler() -> anyhow::Result<()> {
  with_server!(
    "/hello" => "<body><h1>Hello World</h1></body>",
    |base_url| {
        // Combine base_url with relative path
      with_page! {
        base_url, "/hello", |page| {
          assert_has_text!(page.locator("h1"), "Hello World");
        }
      }
    }
  )
}
```

## Architecture

-   **Backends**: Abstracts over CDP (`chromiumoxide`) and WebDriver (`thirtyfour`).
-   **Page**: The main entry point. Currently defaults to Chromium (CDP) when using `with_page!`.
-   **Locator**: Lazy reference to elements. Elements are located only when an action (like assertion) is performed.

## Safe and Hermetic

Using `with_page!` ensures that:
1.  A new browser context/page is created for the test.
2.  The page navigates to the target URL.
3.  The page is closed after the closure completes or panics (cleanup logic is handled).