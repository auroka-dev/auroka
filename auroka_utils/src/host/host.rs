use once_cell::sync::Lazy;
use std::{env, process::Command, sync::Mutex};

static HOST: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(host()));
static HOST_ARCH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(host_arch()));

fn host() -> String {
  let output = Command::new("rustc")
    .arg("-vV")
    .output()
    .expect("Failed to execute command");

  if output.status.success() {
    let output_str = String::from_utf8(output.stdout).unwrap();
    for line in output_str.lines() {
      if line.starts_with("host: ") {
        return line.split(' ').next_back().unwrap().to_string();
      }
    }
  }

  "unknown".to_string()
}

fn host_arch() -> String {
  let host = Host::host();
  host.split('-').next().unwrap_or("unknown").to_string()
}

pub struct Host {}

impl Host {
  pub fn arch() -> String {
    HOST_ARCH.lock().unwrap().clone()
  }

  pub fn ci() -> bool {
    env::var("CI").is_ok_and(|v| v == "true")
  }

  pub fn host() -> String {
    HOST.lock().unwrap().clone()
  }

  pub fn os() -> String {
    std::env::consts::OS.to_string()
  }
}
