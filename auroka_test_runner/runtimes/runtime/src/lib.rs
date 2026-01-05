wit_bindgen::generate!({
    world: "runtime",
    path: "wit",
    pub_export_macro: true,
    default_bindings_module: "auroka_test_runner_runtime_runtime",
});

pub use exports::auroka_test_runner_runtime::runtime::test_runner::Guest as Runtime;
