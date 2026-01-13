use syn::{Block, LitStr};

pub(crate) struct Outcome {
  name: LitStr,
  block: Block,
}

impl Outcome {
  pub fn new(name: LitStr, block: Block) -> Self {
    Outcome { name, block }
  }

  pub fn name(&self) -> &LitStr {
    &self.name
  }

  pub fn block(&self) -> &Block {
    &self.block
  }

  pub fn block_mut(&mut self) -> &mut Block {
    &mut self.block
  }
}
