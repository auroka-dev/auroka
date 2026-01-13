use crate::Outcome;
use syn::{Block, LitStr, Result, parse::ParseStream};

pub(crate) fn parse_outcome(input: ParseStream) -> Result<Outcome> {
  let name: LitStr = input.parse()?;
  let block: Block = input.parse()?;
  Ok(Outcome::new(name, block))
}
