use crate::workerd::start_workerd;
use crate::workerd::wasi32;
use crate::workerd::wasm32;
use anyhow::Result;
use auroka_test_runner_core::BundleKind;
use std::path::Path;
use tempfile::NamedTempFile;
use tokio::process::Child;

pub struct Workerd {
  child: Child,
  config: Vec<NamedTempFile>,
}

impl Workerd {
  pub fn new(config: Vec<NamedTempFile>, child: Child) -> Self {
    Self { child, config }
  }

  pub async fn kill(&mut self) -> Result<()> {
    self.child.kill().await?;
    Ok(())
  }

  pub async fn serve(assembly_path: &Path, port: u16, kind: BundleKind) -> Result<Workerd> {
    let config = match kind {
      BundleKind::Wasi32 => wasi32::generate_config(assembly_path, port)?,
      BundleKind::Wasm32 => wasm32::generate_config(assembly_path, port)?,
    };

    let config_path = config[0].path().to_path_buf();

    Ok(Workerd::new(config, start_workerd(config_path).await?))
  }

  pub async fn wait(&mut self) -> Result<()> {
    let status = self.child.wait().await?;
    if !status.success() {
      anyhow::bail!("Workerd exited with status: {}", status);
    }
    Ok(())
  }

  pub fn child(&mut self) -> &mut Child {
    &mut self.child
  }

  pub fn config(&self) -> &[NamedTempFile] {
    &self.config
  }
}
