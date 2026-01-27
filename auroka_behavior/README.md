# auroka_behavior

`auroka_behavior` is a procedural macro crate that enables Behavior-Driven Development (BDD) style testing in Rust. It provides a structured way to define tests using `Given-When-Then` syntax, automatically expanding them into valid Rust test functions.

## Features

- **BDD Syntax**: specialized `behavior!` macro for clear, readable test specifications.
- **Async Support**: seamless support for async test steps.
- **Multi-Outcome Support**: verify multiple independent outcomes in a single behavior context.
- **Error Accumulation**: collects all assertion failures in a behavior block before failing the test (soft assertions).

## Usage

```rust
use auroka::behavior;

behavior! {
  given_there_is_a_paginated_list()
  when_the_page_changes()

  // Describes the UI result
  "The page content updates" {
     then_the_page_should_change()
  }

  // Describes the invisible system result
  "The pagination event is tracked" {
    then_the_pagination_event_should_be_detected()
  }
}
```

## Structure

The macro expects a specific folder structure for tests:
- `__behaviors__/`: Contains the behavior definitions.
- `__steps__/`: Contains the implementation of steps (`given`, `when`, `then` functions).
- `Context`: A struct to hold test state, passed between steps.

## License

This project is part of the `auroka` workspace.
