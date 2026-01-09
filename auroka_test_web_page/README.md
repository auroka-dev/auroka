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

## Usage

### Basic Navigation and Content Assertion

```rust
use auroka_test_web_page::{with_page, expect};

#[auroka::test]
async fn test_example_com() -> anyhow::Result<()> {
    // Pass a full URL to navigate explicitly
    with_page!("https://example.com", |page| {
        let content = page.content().await?;
        assert!(content.contains("Example Domain"));
    })
}
```

### Locators and Declarative Assertions

```rust
use auroka_test::web::page::{assert_has_text, with_page};

#[auroka::test]
async fn test_element_text() -> anyhow::Result<()> {
    with_page!("https://example.com", |page| {
        // Find element and assert text content using macro
        assert_has_text!(page.locator("h1"), "Example Domain");
    })
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
            with_page!(base_url, "/hello", |page| {
                 assert_has_text!(page.locator("h1"), "Hello World");
            })
        }
    )
}
```

## Architecture

-   **Backends**: Abstracts over CDP (`chromiumoxide`) and WebDriver (`thirtyfour`).
-   **Page**: The main entry point. Currently defaults to Chromium (CDP) when using `with_page!`.
-   **Locator**: Lazy reference to elements. Elements are located only when an action (like assertion) is performed.

## Dependencies

-   **Chromium**: Required for the default backend. The library looks for a local Chrome/Chromium installation.
-   **WebDriver**: If using Firefox or Safari backends explicitly, their respective drivers must be running.

## Safe and Hermetic

Using `with_page!` ensures that:
1.  A new browser context/page is created for the test.
2.  The page navigates to the target URL.
3.  The page is closed after the closure completes or panics (cleanup logic is handled).
