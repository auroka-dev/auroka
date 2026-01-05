use super::Context;
use std::io::{Read, Seek, SeekFrom};

pub fn then_the_config_should_contain(context: &mut Context, expected: &str) {
  let config = context.config();

  assert!(
    config.is_some(),
    "Expected config to be generated, but it was not."
  );

  if let Some(config) = config {
    let mut config_file = &config[0];

    config_file
      .seek(SeekFrom::Start(0))
      .expect("Failed to seek to start of config file");

    let mut content = String::new();
    config_file
      .read_to_string(&mut content)
      .expect("Failed to read config file");

    assert!(
      content.contains(expected),
      "Expected config to contain:\n{}\n\nBut got:\n{}",
      expected,
      content
    );
  }
}
