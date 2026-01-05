use super::Context;
use crate::workerd::wasm32::generate_config;
use std::path::Path;

pub fn when_the_config_is_generated(context: &mut Context) {
  let result = generate_config(Path::new(context.assembly_path()), context.port());

  if let Ok(config) = result {
    context.config_set(config);
  } else if let Err(e) = result {
    context.error_set(e);
  }
}
