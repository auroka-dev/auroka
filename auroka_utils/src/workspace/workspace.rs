use auroka_utils_filesystem::Directory;
use once_cell::sync::Lazy;
use std::{env, path::Path, process::Command, sync::Mutex};

static WORKSPACE_TARGET: Lazy<Mutex<Directory>> = Lazy::new(|| Mutex::new(workspace_target()));
static WORKSPACE_ROOT: Lazy<Mutex<Directory>> = Lazy::new(|| Mutex::new(workspace_root()));

fn workspace_target() -> Directory {
  let mut target = env::current_exe().unwrap();
  target.pop();
  if target.ends_with("deps") {
    target.pop();
    target.pop(); // profile - debug or release
  }
  Directory::new(target)
}

fn workspace_root() -> Directory {
  let output = Command::new(env!("CARGO"))
    .arg("locate-project")
    .arg("--workspace")
    .arg("--message-format=plain")
    .output()
    .unwrap()
    .stdout;

  let path = std::str::from_utf8(&output).unwrap().trim();
  let path = Path::new(path).parent().unwrap();

  Directory::new(path.to_path_buf())
}

pub struct Workspace {}

impl Workspace {
  pub fn target_dir() -> Directory {
    WORKSPACE_TARGET.lock().unwrap().clone()
  }
  pub fn root_dir() -> Directory {
    WORKSPACE_ROOT.lock().unwrap().clone()
  }
}
