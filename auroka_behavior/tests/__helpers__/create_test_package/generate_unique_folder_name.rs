use rand::{Rng, distr::Alphanumeric};

pub(crate) fn generate_unique_folder_name() -> String {
  let rng = rand::rng();
  let suffix: String = rng
    .sample_iter(&Alphanumeric)
    .take(5)
    .map(char::from)
    .collect();
  format!("macro_auroka_test_tests_{}", suffix)
}
