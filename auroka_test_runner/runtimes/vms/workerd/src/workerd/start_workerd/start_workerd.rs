use anyhow::{Context, Result};
use std::{path::PathBuf, process::Stdio};
use tokio::process::Command;

pub async fn start_workerd(config_path: PathBuf) -> Result<tokio::process::Child> {
  println!("Starting workerd with config: {:?}", config_path);

  let child = Command::new("workerd")
    .arg("serve")
    .arg("--verbose")
    .arg(&config_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .kill_on_drop(true) // Ensure we don't leave zombie processes
    .spawn()
    .context("Failed to spawn workerd binary. Is it in your PATH?")?;

  Ok(child)
}
