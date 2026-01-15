use std::io::Write;
use std::process::{Command, Stdio};

pub fn try_format_code(code: &str) -> Option<String> {
  let child = Command::new("rustfmt")
    .arg("--emit=stdout")
    .arg("--config=tab_spaces=2,use_small_heuristics=Max")
    .arg("--edition=2021")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn();

  if let Ok(mut child) = child {
    if let Some(mut stdin) = child.stdin.take() {
      let code = code.to_string();
      // Write input ignoring broken pipe errors
      let _ = std::thread::spawn(move || {
        let _ = stdin.write_all(code.as_bytes());
      });
    }

    if let Ok(output) = child.wait_with_output() {
      if output.status.success() {
        if let Ok(formatted) = std::str::from_utf8(&output.stdout) {
          return Some(formatted.to_string());
        }
      }
    }
  }

  None
}

pub fn format_code(code: &str) -> String {
  try_format_code(code).unwrap_or_else(|| code.to_string())
}
