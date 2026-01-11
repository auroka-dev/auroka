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

### Supported Configurations

`auroka` separates the concepts of **Browser** (Software), **Device** (Hardware), **Operating System** (OS), and **Execution Mode** (Native/Simulated/Emulated).

This allows for flexible testing scenarios, from checking layout on a specific device using desktop emulation to running tests on real hardware.

#### 1. Native Execution
Running the browser naturally on the host OS or a connected device's primary OS.

| Browser                 | Variant Enum                       | OS Support    | Backend   | Requirements                                        |
| :---------------------- | :--------------------------------- | :------------ | :-------- | :-------------------------------------------------- |
| **Chromium** (Default)  | `Browser::Chromium`                | Win, Mac, Lin | CDP       | Chromium Installed.                                 |
| **Google Chrome**       | `Browser::Chrome`                  | Win, Mac, Lin | CDP       | Chrome Installed.                                   |
| **Firefox**             | `Browser::Firefox`                 | Win, Mac, Lin | WebDriver | `geckodriver` & Firefox installed.                  |
| **Safari**              | `Browser::Safari`                  | macOS         | WebDriver | `safaridriver` (Built-in).                          |
| **Safari Tech Preview** | `Browser::SafariTechnologyPreview` | macOS         | WebDriver | `safaridriver` (via STP Bundle).                    |
| **Edge**                | `Browser::Edge`                    | Win, Mac, Lin | CDP       | Edge installed.                                     |
| **Opera**               | `Browser::Opera`                   | Win, Mac, Lin | CDP       | Opera installed.                                    |
| **Brave**               | `Browser::Brave`                   | Win, Mac, Lin | CDP       | Brave installed.                                    |
| **Vivaldi**             | `Browser::Vivaldi`                 | Win, Mac, Lin | CDP       | Vivaldi installed.                                  |
| **Arc**                 | `Browser::Arc`                     | Mac, Win      | CDP       | Arc installed.                                      |
| **WebKit** (Epiphany)   | `Browser::WebKit`                  | Linux, Mac    | WebDriver | `webkit-webdriver` (Linux), `safaridriver` (macOS). |
| **Electron**            | `Browser::Electron`                | Win, Mac, Lin | WebDriver | `chromedriver` targeting app binary.                |

#### 2. Simulated Execution (Software)
Uses a desktop browser (usually Chrome via CDP) to **mimic** mobile/tablet devices by overriding Viewport, User-Agent, Touch Events, and Pixel Ratio. This is fast but less accurate than hardware emulation.

**Common Syntax**: `with_page! { :Chrome on :Pixel7 }`

| Device Type       | Specific Devices (Variant Enum)                                                                                        | Default Backend |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------- | :-------------- |
| **Phones**        | `:Pixel7`, `:GalaxyS23`, `:IPhone14Pro`, `:IPhone14ProMax`, `:IPhone13Mini`, `:IPhone8`, `:IPhoneSE`                   | CDP             |
| **Tablets**       | `:GalaxyTabS9`, `:IPadPro`, `:IPadAir`, `:IPadMini`, `:IPad`                                                           | CDP             |
| **Foldables**     | `:GalaxyFoldInner` (Unfolded), `:GalaxyFoldOuter` (Folded)                                                             | CDP             |
| **TV / STB**      | `:AppleTV4K` (4K), `:AppleTV` (HD), `:AndroidTV4K`, `:AndroidTV`, `:FireTV`, `:Xbox`, `:PlayStation4`, `:PlayStation5` | CDP             |
| **Smart Display** | `:NestHub`, `:EchoShow`                                                                                                | CDP             |
| **Wearables**     | `:AppleWatch`                                                                                                          | CDP             |
| **XR / Spatial**  | `:VisionOS` (Simulated viewport)                                                                                       | CDP             |

#### 3. Emulated Execution (Hardware)
Running tests on a virtualized Operating System (e.g., Android Emulator (AVD), iOS Simulator, Xcode).

| Platform     | Browser Implementation      | Backend   | Requirements                              |
| :----------- | :-------------------------- | :-------- | :---------------------------------------- |
| **Android**  | `Browser::ChromeMobile`     | WebDriver | Android SDK + AVD + `chromedriver`        |
| **Android**  | `Browser::FirefoxMobile`    | WebDriver | Android SDK + AVD + `geckodriver`         |
| **Android**  | `Browser::EdgeMobile`       | WebDriver | Android SDK + AVD + `msedgedriver`        |
| **Android**  | `Browser::OperaMobile`      | WebDriver | Android SDK + AVD + `operadriver`         |
| **iOS**      | `Browser::SafariMobile`     | WebDriver | Xcode + iOS Simulator                     |
| **tvOS**     | `Browser::Safari :AppleTV`  | WebDriver | Xcode + tvOS Simulator (via safaridriver) |
| **visionOS** | `Browser::Safari :VisionOS` | WebDriver | Xcode + visionOS Simulator                |

### Operating System Support

You can now explicitly specify the target OS in your test configuration, which is helpful when connecting to a remote grid or ensuring the environment matches.

```rust
use auroka_test_web_browser_shared::OperatingSystem;

// Explicitly requesting Windows
with_page! { :Chrome :Windows ... }

// Requesting Android explicitly
with_page! { :Chrome on :Pixel7 :Android ... }
```

| OS Enum                     | Description                       |
| :-------------------------- | :-------------------------------- |
| `OperatingSystem::Windows`  | Microsoft Windows                 |
| `OperatingSystem::MacOS`    | Apple macOS                       |
| `OperatingSystem::Linux`    | Linux Distributions               |
| `OperatingSystem::Android`  | Google Android (Mobile/TV/Auto)   |
| `OperatingSystem::IOS`      | Apple iOS (iPhone/iPad)           |
| `OperatingSystem::TvOS`     | Apple tvOS (Apple TV)             |
| `OperatingSystem::WatchOS`  | Apple watchOS (Apple Watch)       |
| `OperatingSystem::VisionOS` | Apple visionOS (Apple Vision Pro) |

## Test Coverage Matrix

The following table shows test coverage across all browsers and platforms. Each platform is subdivided into three execution modes: Emulator, Native, and Simulator.

**Legend**: ✅ Supported | ❓ Test exist | Nat - Native | Em - Emulator | Sim - Simulator

<table>
<thead>
<tr>
<th rowspan="2">Browser</th>
<th colspan="3" style="border-left: 2px solid #ddd;">Android</th>
<th colspan="3" style="border-left: 2px solid #ddd;">ChromeOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">iOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">iPadOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">Linux</th>
<th colspan="3" style="border-left: 2px solid #ddd;">macOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">Windows</th>
<th colspan="3" style="border-left: 2px solid #ddd;">tvOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">visionOS</th>
<th colspan="3" style="border-left: 2px solid #ddd;">watchOS</th>
</tr>
<tr>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
<th style="border-left: 2px solid #ddd;">Nat</th><th>Em</th><th>Sim</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Arc</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Brave</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Chrome</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Chromium</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Edge</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Electron</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Firefox</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Opera</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Safari</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
</tr>
<tr>
<td><strong>Safari Tech Preview</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">✅</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>Samsung Internet</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>UC Browser</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<tr>
<td><strong>Vivaldi</strong></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
</tr>
<tr>
<td><strong>WebKit</strong></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td></td><td></td>
<td style="border-left: 2px solid #ddd;"></td><td></td><td></td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
<td style="border-left: 2px solid #ddd;">❓</td><td>❓</td><td>✅</td>
</tr>
</tbody>
</table>

### Browser Setup Details

-   **Chromium vs Chrome**: `Browser::Chromium` strictly looks for a Chromium binary (e.g., `/usr/bin/chromium`). It will not launch Google Chrome. Use `Browser::Chrome` for Google Chrome.
-   **Safari Technology Preview**: Requires the "Safari Technology Preview" app to be installed in `/Applications`. The system automatically targets the embedded `safaridriver` inside the app bundle.
-   **WebKit (macOS)**:
    -   **Default**: Uses system `safaridriver`. Run `safaridriver --enable`.
    -   **Native WebKit**: If you have a custom `webkit-webdriver` binary in your `$PATH` (e.g., from a WebKit build), `auroka` will prefer it over Safari.
-   **WebKit (Linux)**: Requires `webkit2gtk-driver`. Install via your package manager (e.g., `sudo apt install webkit2gtk-driver`).
-   **Android**: Ensure `adb` is in your PATH and an emulator/device is connected.

#### WebKit (Parallel / CDP)
For faster, parallel WebKit testing, `auroka` supports **Playwright's WebKit** (a patched version that speaks Chrome DevTools Protocol).

1.  **Install**:
    ```bash
    npx playwright install webkit
    ```
    (Ensure the `ms-playwright` cache is in the standard location: `~/Library/Caches/ms-playwright` on macOS/Linux).

2.  **Usage**:
    Use the `:Simulator` tag to opt-in to this backend.
    ```rust
    with_page! { :WebKit :Simulator
      // Runs in Playwright WebKit via CDP (Parallel, Fast)
    }
    ```

#### WebKit on macOS (Native / WebDriver)
On macOS, the **Safari Driver** (`safaridriver`) acts as the native WebKit implementation. You generally do not need a separate binary.

1.  **Enable Automation**:
    ```bash
    safaridriver --enable
    ```
2.  **Usage**:
    Use `:Native` (or default if no mode specified) to use the system driver.
    ```rust
    with_page! { :WebKit :Native
      // Runs in Safari Driver (Serial, Standard)
    }
    ```

**Note regarding Playwright**:
While Playwright downloads a WebKit bundle, it uses a patched version allowing CDP (Chrome Protocol) connections. It generally **does not** include a standard `webkit-webdriver` binary. To test "Pure WebKit" behavior via WebDriver on macOS, using the system Safari or Safari Technology Preview is the standard approach.

### Installation (macOS via Homebrew)

```bash
# Browsers
brew install --cask google-chrome firefox microsoft-edge opera brave-browser vivaldi arc

# Drivers
brew install chromedriver geckodriver msedgedriver operadriver
```

**Safari**: Run `safaridriver --enable`.

**Android**: Install Android Studio, ensure `adb` is in PATH, and start an emulator.

**Emulated Devices**: Devices marked as `(Emulated)` or `(Emul)` in the tables above (e.g., PlayStation, iPhone Emulation) run inside a standard headless desktop browser (Chromium/Chrome) and do **not** require simulators, SDKs, or additional drivers. They work out-of-the-box.

## Performance & Strategy: Staged Testing

For maximum velocity, we recommend a **staged testing pipeline**. Start with fast, high-concurrency emulation (CDP), and only use heavy simulators (WebDriver) for final verification.

### Stage 1: The "Speed" Layer (CDP Emulation)
Run these **checks on every commit**. They launch instantly, run in parallel, and catch 90% of layout/JS bugs.
*   **Browsers**: `Browser::Chromium`, `Browser::ChromeMobileEmulation`.
*   **Alternative Browsers**: `Browser::Brave` (Desktop), `Browser::Vivaldi` (Desktop), `Browser::BraveMobileEmulation`, `Browser::SamsungInternetEmulation`.
*   **Phones (Responsive)**: `Browser::IPhoneSEEmulation`, `Browser::IPhone14ProEmulation`, `Browser::Pixel7Emulation`, `Browser::GalaxyS23Emulation`, `Browser::GalaxyFoldInnerEmulation`.
*   **TV**: `AppleTVEmulation` (HD), `AppleTV4KEmulation`, `AndroidTVEmulation` (HD), `AndroidTV4KEmulation`.
*   **Tablets & Wearables**: `IPadEmulation`, `IPadMiniEmulation`, `IPadAirEmulation`, `AppleWatchEmulation`.
*   **Smart Displays**: `Browser::NestHubEmulation`, `Browser::EchoShowEmulation`.
*   **Gaming Consoles**: `Browser::PlayStation4Emulation`, `Browser::PlayStation5Emulation`, `Browser::XboxEmulation`, `Browser::FireTVEmulation`.
*   **Backend**: Headless Chromium (CDP).
*   **Speed**: ~Milliseconds per test.

### Stage 2: The "Compatibility" Layer
Run these **before merge**. Catch engine-specific rendering issues.
*   **Browsers**: `Browser::Firefox`, `Browser::FirefoxMobile`, `Browser::WebKit` (GNOME Web).
*   **Backend**: WebDriver.
*   **Speed**: ~Seconds per test.

### Stage 3: The "Strict" Layer (Native Simulators)
Run these **nightly or on release**. Catch platform-specific input/OS quirks.
*   **Browsers**: `Browser::Safari` (macOS), `Browser::SafariMobile` (iOS Simulator), `Browser::AppleTV` (tvOS Simulator), `Browser::VisionOS` (Simulator).
*   **Backend**: WebDriver (Serial execution required for Simulators).
*   **Speed**: ~Minutes per suite (Due to boot times and single-concurrency locks).

## Architecture & Throttling (Important)

`auroka_test_web_page` relies on a local infrastructure to manage browser concurrency and prevent resource exhaustion.

1.  **Central Registry**: When you run a test, `auroka` automatically spawns a `Registry` (Port 3000) and a `Node` (Port 8081).
2.  **Concurrency Limits**:
    *   **Safari**: Max 1 simultaneous session (System Limitation).
    *   **Firefox/Chrome**: Configured limits (default ~8-10).
3.  **Throttling**: If you run `cargo test -j 16`, 16 tests will start, but they will **block and queue** at the `Page::new()` step until a browser slot is available.
    *   This applies to **all** browsers, including local **Chromium (CDP)**.
4.  **Network Resilience**: The system uses a persistent "Keep-Alive" heartbeat. If your test process crashes, the Registry detects the connection drop and instantly releases the browser slot for other tests.

## Usage

### Network Configuration (Mobile/Emulator)

When testing on Android Emulators or Containers, `localhost` on the host machine is often not accessible as `localhost` inside the emulator.

-   **Automatic Handling**: For Android browsers (`Browser::ChromeMobile`, etc.), `auroka` automatically rewrites `127.0.0.1` and `localhost` to `10.0.2.2` (standard Android Emulator host alias).
-   **Custom Alias**: If you are using Genymotion, Docker, or a real device, you can override this behavior:

```rust
use auroka_test_web_page::{PageConfig, Page};

let config = PageConfig::new()
    .browser(Browser::ChromeMobile)
    .localhost_alias("192.168.1.100"); // IP of your host machine

let page = Page::launch(config).await?;
```

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
