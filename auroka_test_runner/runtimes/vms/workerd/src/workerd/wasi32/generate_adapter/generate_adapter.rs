use anyhow::Result;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn generate_adapter() -> Result<NamedTempFile> {
  // This is the glue code that imports the WASM component and exposes it to Workerd's JS runtime.
  let mut adapter_file = NamedTempFile::new()?;
  let adapter_content = r#"
// Generic Adapter for WASI Components
import * as component from "component";

export default {
  async fetch(request, env, ctx) {
    // Basic pass-through to the component's incoming-handler
    // We assume the component exports the standard wasi:http/incoming-handler interface
    // which usually maps to a 'handle' function in the binding.
    if (component.handle) {
        return component.handle(request);
    } else if (component.incomingHandler && component.incomingHandler.handle) {
        return component.incomingHandler.handle(request);
    } else {
        return new Response("Adapter Error: Component does not export a known handler.", { status: 500 });
    }
  }
};
"#;
  adapter_file.write_all(adapter_content.as_bytes())?;

  Ok(adapter_file)
}
