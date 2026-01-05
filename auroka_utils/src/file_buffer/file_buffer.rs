#[derive(Debug)]
pub struct FileBuffer {
  content: String,
  name: String,
}

impl FileBuffer {
  pub fn new<T: Into<String>, U: Into<String>>(name: T, content: U) -> Self {
    FileBuffer {
      name: name.into(),
      content: content.into(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}
