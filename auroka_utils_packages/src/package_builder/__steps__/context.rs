use crate::PackageBuilder;

pub struct Context {
  error: Option<String>,
  output: Option<String>,
  package_builder: Option<PackageBuilder>,
  result: Option<Result<(), std::io::Error>>,
}

impl Context {
  pub fn new() -> Self {
    Context { error: None, output: None, package_builder: None, result: None }
  }

  pub fn error(&self) -> &Option<String> {
    &self.error
  }

  pub fn error_set(&mut self, error: Option<String>) {
    self.error = error;
  }

  pub fn output(&self) -> &Option<String> {
    &self.output
  }

  pub fn output_set(&mut self, output: Option<String>) {
    self.output = output;
  }

  pub fn package_builder_mut(&mut self) -> &mut PackageBuilder {
    self.package_builder.as_mut().unwrap()
  }

  pub fn package_builder_set(&mut self, package_builder: PackageBuilder) {
    self.package_builder = Some(package_builder);
  }

  pub fn result(&self) -> &Result<(), std::io::Error> {
    self.result.as_ref().unwrap()
  }

  pub fn result_set(&mut self, result: Result<(), std::io::Error>) {
    self.result = Some(result);
  }
}
