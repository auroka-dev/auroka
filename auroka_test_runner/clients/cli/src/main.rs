mod invocation;
mod output_format;
mod run;
mod test_report;

pub use invocation::Invocation;
pub use output_format::OutputFormat;
pub use run::run;
pub use test_report::TestReport;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
  auroka_test_runner::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
  println!("This binary is not supported on WASM.");
}
