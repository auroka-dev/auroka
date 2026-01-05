use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Directory {
  path: PathBuf,
}

impl Directory {
  pub fn new(path: PathBuf) -> Self {
    Directory { path }
  }

  pub fn exists_file(&self, name: &str) -> bool {
    let file_path = self.path.join(name);
    file_path.is_file()
  }

  pub fn join(&self, name: &str) -> Directory {
    Directory::new(self.path.join(name))
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn parent(&self) -> Directory {
    Directory::new(self.path.parent().unwrap().to_path_buf())
  }

  pub fn remove(&self) -> Result<(), std::io::Error> {
    fs::remove_dir_all(&self.path)
  }

  pub fn read_file(&self, name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(self.path.join(name))
  }

  pub fn reset(&self) -> Result<(), std::io::Error> {
    drop(self.remove());
    fs::create_dir_all(&self.path)
  }

  pub fn strip_prefix(&self, prefix: &Directory) -> Directory {
    Directory::new(self.path.strip_prefix(&prefix.path).unwrap().to_path_buf())
  }

  pub fn write_file(&self, name: &str, content: &str) -> Result<(), std::io::Error> {
    let target = self.path.join(name);
    let parent = target.parent().unwrap();
    fs::create_dir_all(parent)?;
    fs::write(target, content)
  }
}

impl Display for Directory {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    self.path.display().fmt(f)
  }
}
