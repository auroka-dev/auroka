use super::super::generate_adapter;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn generate_config(assembly_path: &Path, port: u16) -> Result<Vec<NamedTempFile>> {
  let path_str = assembly_path.to_str().context("Invalid path")?;

  let path_prefix = path_str.trim_end_matches(".wasm");

  let adapter_file = generate_adapter()?;

  let adapter_path_str = adapter_file
    .path()
    .to_str()
    .context("Invalid adapter path")?;

  let mut config_file = NamedTempFile::new()?;

  let config = format!(
    r#"
using Workerd = import "/workerd/workerd.capnp";

const config :Workerd.Config = (
  services = [
    (name = "main", worker = .mainWorker),
    (name = "internet", network = ( allow = ["public", "private"] )),
  ],
  sockets = [
    ( name = "http", address = "*:{port}", http = (), service = "main" )
  ]
);

const mainWorker :Workerd.Worker = (
  modules = [
    # 1. The Adapter is the Entry Point
    (name = "main", esModule = embed "adapter.js"),
    
    # 2. The Bundled JCO Worker (Imported by adapter)
    (name = "{adapter_path_str}", esModule = embed "{adapter_path_str}"),
    
    # 3. The WASM Files (Ensure these match your patch-workerd.js output)
    (name = "{path_prefix}.core.wasm", wasm = embed "{path_prefix}.core.wasm"),
    (name = "{path_prefix}.internal.0.wasm", wasm = embed "{path_prefix}.internal.0.wasm"),
    (name = "{path_prefix}.internal.1.wasm", wasm = embed "{path_prefix}.internal.1.wasm"),
  ],
  
  compatibilityDate = "2024-08-16",
  compatibilityFlags = ["nodejs_compat"],
  globalOutbound = "internet",
);
"#,
    port = port,
    path_prefix = path_prefix,
    adapter_path_str = adapter_path_str
  );

  config_file.write_all(config.as_bytes())?;

  Ok(vec![config_file, adapter_file])
}
