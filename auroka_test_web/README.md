# auroka_test_web

Web testing utilities for Auroka.

## Features

- `with_server!`: Creates a lightweight HTTP server for testing.

## Usage

### `with_server!`

The `with_server!` macro spins up a temporary HTTP server on a random port for the duration of the test block.

```rust
use auroka_test_web::with_server;
use serde_json::json;

#[tokio::test]
async fn test_my_server() {
    with_server!(
        // Simple HTML string (defaults to 200 OK, text/html)
        "/html" => "<h1>Hello</h1>",
        
        // JSON response (defaults to 200 OK, application/json)
        "/api/json" => json!({ "status": "ok" }),
        
        // Custom status code with body (Content-Type defaults to text/html)
        "/not-found" => (404, "Custom Not Found"),
        
        // Full control: Status, Content-Type, Body
        "/custom" => (201, "text/plain", "Created");
        
        |base_url| {
            // base_url is "http://127.0.0.1:<port>"
            // Your test code here...
        }
    )
}
```
