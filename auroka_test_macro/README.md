# auroka_test_macro

A procedural macro attribute for defining `auroka` tests.

This crate provides the `#[auroka_test]` attribute, which:
1.  Transforms an async test function into a synchronous `#[test]` wrapper that sets up a `tokio` runtime.
2.  Registers the test function in a global `inventory` for discovery by the `auroka_test_runner`.

## Usage

```rust
use auroka_test_macro::auroka_test;

#[auroka_test]
async fn my_test() {
    // Async test code here
}
```

## Architecture

The macro expansion process involves:
-   **Extraction**: Accessing the original function body.
-   **Transformation**: Generating an inner `async` function (if applicable) and an outer synchronous `#[test]` wrapper.
-   **Registration**: Generating an `inventory::submit!` block to register the test case.
