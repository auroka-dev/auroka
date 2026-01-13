use crate::Outcome;
use syn::{
  Block, Expr, ExprCall, LitStr, Result,
  parse::{Parse, ParseStream},
};

pub(crate) struct Behavior {
  is_async: bool,
  setup_steps: Vec<ExprCall>,
  outcomes: Vec<Outcome>,
}

impl Behavior {
  pub fn new() -> Self {
    Behavior { is_async: false, setup_steps: Vec::new(), outcomes: Vec::new() }
  }

  pub fn is_async(&self) -> bool {
    self.is_async
  }

  fn is_async_set(&mut self, value: bool) {
    self.is_async = value;
  }

  pub fn setup_steps_mut(&mut self) -> &mut Vec<ExprCall> {
    &mut self.setup_steps
  }

  pub fn outcomes(&self) -> &Vec<Outcome> {
    &self.outcomes
  }

  pub fn outcomes_mut(&mut self) -> &mut Vec<Outcome> {
    &mut self.outcomes
  }
}

impl Parse for Behavior {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut behavior = Behavior::new();

    if input.peek(syn::Token![:]) {
      input.parse::<syn::Token![:]>()?;
      input.parse::<syn::Token![async]>()?;
      behavior.is_async_set(true);
    }

    while !input.is_empty() {
      if input.peek(LitStr) {
        // Parse a test case: "Name" { ... }
        let name: LitStr = input.parse()?;
        let block: Block = input.parse()?;
        behavior.outcomes_mut().push(Outcome::new(name, block));
      } else {
        // Parse a setup step: function_call()
        // We parse it as an expression first to handle the call syntax
        let expr: Expr = input.parse()?;
        if let Expr::Call(call) = expr {
          behavior.setup_steps_mut().push(call);
        } else {
          return Err(syn::Error::new_spanned(expr, "Expected a function call for setup step"));
        }
      }
    }

    Ok(behavior)
  }
}
