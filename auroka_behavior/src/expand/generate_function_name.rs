use crate::Outcome;
use quote::format_ident;
use syn::Ident;

pub(crate) fn generate_function_name(outcomes: &[Outcome]) -> Ident {
  let fn_name_str = if outcomes.is_empty() {
    "no_outcome".to_string()
  } else if outcomes.len() == 1 {
    outcomes[0]
      .name()
      .value()
      .to_lowercase()
      .replace(" ", "_")
      .replace("-", "_")
  } else {
    "compact".to_string()
  };
  format_ident!("{}", fn_name_str)
}
