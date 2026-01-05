use inventory;
use std::future::Future;
use std::pin::Pin;

pub type TestFn = fn() -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

pub struct Test {
  pub name: &'static str,
  pub test_fn: TestFn,
}

// This macro allows the registry to collect items of type `Test`
inventory::collect!(Test);

pub fn get_tests() -> impl Iterator<Item = &'static Test> {
  inventory::iter::<Test>.into_iter()
}

pub trait TestReturn {
  fn into_result(self) -> anyhow::Result<()>;
}

impl TestReturn for () {
  fn into_result(self) -> anyhow::Result<()> {
    Ok(())
  }
}

impl<E> TestReturn for Result<(), E>
where
  E: Into<anyhow::Error>,
{
  fn into_result(self) -> anyhow::Result<()> {
    self.map_err(|e| e.into())
  }
}
