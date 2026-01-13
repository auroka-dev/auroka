#[cfg(feature = "web")]
pub mod web {
  pub mod page {
    pub use auroka_assertions_web_page::*;
    pub use auroka_test_web_page::*;
  }
  pub use auroka_assertions_web::*;
  pub use auroka_test_web::*;
}
pub use auroka_test_macro::*;
pub use auroka_test_registry::*;
#[cfg(feature = "web")]
pub use web::*;
