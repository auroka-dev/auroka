# auroka

A next-generation framework for testing WebAssembly components using WIT (WebAssembly Interface Types) bindings.

## Overview

`auroka` is designed to decouple component logic from specific runtimes. By leveraging WIT bindings, it allows developers to write code that is abstract, testable, and portable.

A major focus of this project is **Test-Driven Development (TDD)** and a robust **Cross-Platform Testing Infrastructure**. We believe that tests should run natively for speed (using simulators) and inside actual WASM environments (Node, Deno, Workerd, Browser) for correctness.

## Getting Started

### Prerequisites
- Rust (latest stable or nightly)

### Running Tests
To run the tests natively (using the simulation host):

```bash
cargo test
```

To see the custom runner output (verifying the registry works):

```bash
cargo test -- --nocapture
```

## Testing Philosophy

We follow a strict "Smallest Step" TDD approach.

### The `#[auroka_test]` Macro

Instead of using the standard `#[test]`, we use `#[auroka::test]`. This provides two benefits:

1.  **Native Execution**: It generates a standard `#[test]` wrapper that runs the code natively using `tokio` and our host simulators. This is fast and works with standard IDE tools.
2.  **WASM Registration**: It registers the test in a global inventory. This allows our custom runner to execute these tests inside WASM environments (like Cloudflare Workers or Node.js) where the standard Rust test harness does not exist.

```rust
#[auroka::test]
async fn test_kv_flow() {
    // This runs natively via `cargo test`
    // AND can be compiled to WASM to run in `workerd`
    assert!(true);
}
```

### Browser Automation

We provide a high-level, Playwright-inspired API for End-to-End (E2E) testing. This allows you to control a browser (Chromium via CDP or others via WebDriver) directly from your Rust tests.

The syntax is designed to be clean and idiomatic, supporting `async/await` and declarative macros for concise assertions.

```rust
use auroka_test::web::{assert_has_text, with_page};

#[auroka::test]
async fn loads_home_in_german() -> anyhow::Result<()> {
  with_page!("/de", |page| {
    assert_has_text!(
      page.locator("footer .socials"),
      "Folgen Sie uns in den sozialen Medien:"
    );
  })
}
```

## Workspace Structure

The project is organized as a Cargo Workspace with the following components:

### SDKs & Core
- **`auroka`**: The primary Rust SDK that developers use to write Workers.
- **`auroka_behavior`**: Behavior Driven Development (BDD) support.
- **`auroka_assertions`**: Specialized assertion macros and utilities.

### Testing Infrastructure
The testing stack is designed to be fully compatible with standard Rust tooling while enabling cross-platform execution.

- **`auroka_test`**: The user-facing library. Exports the `#[auroka_test]` macro.
- **`auroka_test_macro`**: The procedural macro that wraps tests. It generates standard `#[test]` entry points (for `cargo test`) and registers tests in a global inventory.
- **`auroka_test_registry`**: A platform-agnostic registry that collects test definitions at runtime.
- **`auroka_test_runner`**: The test harness library. It contains the logic to iterate over the registry and execute tests (supporting both sync and async).
- **`auroka_test_web`**: High-level browser automation library (Playwright-like API) for E2E testing.

### Utilities
- **`auroka_utils`**: Shared utilities for file buffers, host simulation, and workspace management.
- **`auroka_utils_filesystem`**: Filesystem abstractions.
- **`auroka_utils_packages`**: Package management and generation utilities.

## Roadmap

- [x] **Native Runner**: Basic registry and runner infrastructure working on native targets.
- [x] **Async Support**: Full support for async/await in tests.
- [x] **WASM Compilation**: Pipeline to compile tests to `.wasm` (via `wasm32-wasip2`).

### 1. Core Platform & Runner
- [ ] **Standard Tooling Support**:
    - [ ] **`cargo test`**: Full compatibility for native and WASM targets.
    - [ ] **`cargo nextest`**: Support for parallel test execution and rich reporting.

- [ ] **Multi-Language Support**:
    - [ ] **Rust**: Primary SDK (Current).
    - [ ] **TypeScript/JavaScript**: SDK generation via `jco` and `wit-bindgen`.
    - [ ] **Python**: Support for Pyodide/Componentize-Py.
    - [ ] **Go**: Support for TinyGo/Go WASM.

- [ ] **CLI Runner (`cargo-auroka-test`)**: A unified tool to orchestrate the testing pipeline:
    1.  **Unit Tests**:
        - [ ] *Native*: Runs `cargo test` for instant feedback using simulators.
        - [ ] *WASM*: Compiles to `wasm32-wasip3` and runs in lightweight runtimes (Node/Wasmtime).
    2.  **Integration Tests**:
        - [ ] Deploys the Worker to a real runtime (e.g., local `workerd` instance).
        - [ ] Tests interactions with real or emulated resources (KV, Durable Objects) via HTTP/Bindings.
    3.  **E2E Tests**:
        - [x] **High-Level Browser Automation**:
            - [x] **Chrome DevTools Protocol (CDP)**: Direct integration for high-performance testing without WebDriver overhead.
            - [x] **Playwright-like API**: Rust-native `Page`, `Locator`, and `Expect` abstractions.
            - [ ] **Cross-Browser Support**:
                - [ ] Chromium (Chrome/Edge) via CDP.
                - [ ] Firefox/WebKit (Planned).
        - [ ] Verifies the full stack from UI down to the Worker.

- [ ] **Platform Support** (Target Runtimes):
    - [ ] **Edge/Serverless**:
        - [ ] Bun
        - [ ] Deno
        - [ ] Fastly Compute
        - [ ] Node.js
        - [ ] Spin (Fermyon)
        - [ ] Workerd (Cloudflare)
    - [ ] **Browsers** (WASM Execution):
        - [ ] Chrome
        - [ ] Edge
        - [ ] Firefox
        - [ ] Safari
    - [ ] **Pure WASM / WASI**:
        - [ ] WasmEdge
        - [ ] Wasmer
        - [ ] Wasmtime

### 2. Configuration & Quality
- [ ] **Configuration (`auroka.toml`)**:
    - [ ] **Test Profiles**: Define browser resolutions, device emulation, and headless settings.
    - [ ] **Environment Variables**: Per-environment secrets and settings (Local vs Staging vs Prod).
    - [ ] **Runtime Selection**: Matrix configuration (e.g., "Run unit tests on Node 18 and Wasmtime").

- [ ] **Code Coverage**:
    - [ ] Native coverage via `llvm-cov`.
    - [ ] WASM coverage aggregation across different runtimes.

### 3. Advanced Testing Capabilities
- [ ] **Behavior Driven Development (BDD)**: Support for Gherkin alike syntax (Given/When/Then) via `auroka_behavior` macro to align tests with business requirements.
- [ ] **Property Based Testing**: Integration with `proptest` to automatically generate test cases and verify invariants.
- [ ] **Fuzz Testing**: Integration with `cargo-fuzz` or `arbitrary` to discover edge cases in WIT interfaces.
- [ ] **Network Mocking (VCR)**: Record and replay HTTP interactions to ensure deterministic tests without external dependencies.
- [ ] **Snapshot Testing**: Verify complex JSON responses or HTML output against stored baselines.

### 4. Observability & Performance
- [ ] **Test Observability**:
    - [ ] **OpenTelemetry Integration**: Capture logs and traces from inside the test execution.
    - [ ] **Time Travel Debugging**: (Long term) Record execution states for replay.

- [ ] **Performance & Benchmarking**:
    - [ ] **Regression Testing**: Track execution time, memory usage, and binary size over time.

### 5. Developer Experience & CI
- [ ] **Developer Experience (DX)**:
    - [ ] **Documentation**: Automated generation of API docs and architecture diagrams.
    - [ ] **IDE Integration**: VS Code extension to run/debug individual WASM tests via Code Lenses.
    - [ ] **Watch Mode**: Automatically re-run tests on file changes (`cargo auroka test --watch`).

- [ ] **CI/CD Integration**:
    - [ ] **GitHub Actions**: Pre-built actions to setup the `auroka` environment in CI.
    - [ ] **Flaky Test Detection**: Automatically retry and flag unstable tests.
    - [ ] **Test Sharding**: Distribute tests across multiple CI runners to reduce build times.
