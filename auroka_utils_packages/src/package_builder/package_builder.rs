use crate::{Package, PackageGenerator, environment::Environment};
use auroka_utils::Workspace;
use auroka_utils_filesystem::Directory;
use std::process::Command;

#[derive(Debug)]
pub struct PackageBuilder {
  error: Option<String>,
  generated: bool,
  output: Option<String>,
  package: Package,
  root: Directory,
}

impl PackageBuilder {
  pub fn try_new(folder: &str, package: Package) -> Result<Self, std::io::Error> {
    let root = Workspace::target_dir().join(folder).join(package.name());
    root.reset()?;
    Ok(PackageBuilder { error: None, generated: false, output: None, root, package })
  }

  pub fn build(&mut self) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = Command::new("cargo")
      .current_dir(self.root.path())
      .arg("build")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  fn cargo(&self, enviroment: Environment) -> Command {
    let mut command = Command::new("cargo");

    command.current_dir(self.root.path());

    if let Environment::CI = enviroment {
      command.env("CI", "true");
    } else {
      command.env_remove("CI");
    }

    command
  }

  pub fn expand(&mut self, environment: Environment) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = self.cargo(environment).arg("expand").output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn expand_target(&mut self, environment: Environment, target: &str) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = self
      .cargo(environment)
      .arg("expand")
      .arg("--target")
      .arg(target)
      .arg("--lib")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn expand_test(&mut self, environment: Environment) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = self
      .cargo(environment)
      .arg("expand")
      .arg("--lib")
      .arg("--tests")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn expand_test_target(&mut self, environment: Environment, target: &str) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = self
      .cargo(environment)
      .arg("expand")
      .arg("--target")
      .arg(target)
      .arg("--lib")
      .arg("--tests")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());

    let raw_output = std::str::from_utf8(&output.stdout).unwrap().to_string();

    // Attempt to format the output with rustfmt to respect project settings (2 spaces)
    let formatted_output = auroka_utils::format_code(&raw_output);

    self.output = Some(formatted_output);

    Ok(())
  }

  pub fn execute_test_target(&mut self, environment: Environment, target: &str) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = self
      .cargo(environment)
      .arg("test")
      .arg("--target")
      .arg(target)
      .arg("--lib")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn generate(&mut self) -> Result<(), std::io::Error> {
    if !self.generated {
      PackageGenerator::generate(&self.root, &self.package)?;
      self.generated = true;
    }
    Ok(())
  }

  pub fn test(&mut self) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = Command::new("cargo")
      .current_dir(self.root.path())
      .arg("test")
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn test_target(&mut self, target: &str) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = Command::new("cargo")
      .current_dir(self.root.path())
      .arg("test")
      .arg("--target")
      .arg(target)
      .output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn error(&self) -> &Option<String> {
    &self.error
  }

  pub fn output(&self) -> &Option<String> {
    &self.output
  }
}

impl Drop for PackageBuilder {
  fn drop(&mut self) {
    self.root.parent().remove().unwrap();
  }
}
