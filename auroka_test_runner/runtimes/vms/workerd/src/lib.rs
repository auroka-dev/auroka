#![cfg(not(target_arch = "wasm32"))]
mod workerd;

pub use workerd::Workerd;
