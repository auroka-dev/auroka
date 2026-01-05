use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
  Pretty,
  Terse,
  Json,
}
