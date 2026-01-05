use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn generate_config(assembly_path: &Path, port: u16) -> Result<Vec<NamedTempFile>> {
  let mut file = NamedTempFile::new()?;
  let path_str = assembly_path.to_str().context("Invalid path")?;

  let config = format!(
    r#"
using Workerd = import "/workerd/workerd.capnp";

const config :Workerd.Config = (
  services = [
    (name = "main", worker = .mainWorker),
    (name = "internet", network = ( allow = ["public", "private"] )),
  ],
  sockets = [
    ( name = "http",
      address = "*:{port}",
      http = (),
      service = "main"
    ),
  ]
);

const mainWorker :Workerd.Worker = (
  # Assuming the assembly is compatible with the service worker API or 
  # configured as a module.
  serviceWorkerScript = embed "{path_str}",
  compatibilityDate = "2025-08-16",
);
"#,
    port = port,
    path_str = path_str
  );

  file.write_all(config.as_bytes())?;

  Ok(vec![file])
}
