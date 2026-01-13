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
   given_some_state()
   when_something_happens()

   "The system state should be valid" {
     then_state_should_be_valid()
   }

   "The system should respond correctly" {
     then_response_should_be_ok()
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
