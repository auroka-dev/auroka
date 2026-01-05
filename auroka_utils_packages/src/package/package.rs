use crate::Dependency;
use auroka_utils::FileBuffer;

#[derive(Debug)]
pub struct Package {
  dependencies: Vec<Dependency>,
  files: Vec<FileBuffer>,
  name: String,
}

impl Package {
  pub fn new(name: &str) -> Self {
    Package {
      name: name.to_string(),
      dependencies: Vec::new(),
      files: Vec::new(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn add_dependency(&mut self, dependency: Dependency) {
    self.dependencies.push(dependency);
  }

  pub fn add_file(&mut self, file: FileBuffer) {
    self.files.push(file);
  }

  pub fn dependencies(&self) -> &Vec<Dependency> {
    &self.dependencies
  }

  pub fn files(&self) -> &Vec<FileBuffer> {
    &self.files
  }
}
