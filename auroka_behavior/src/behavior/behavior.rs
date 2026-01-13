use super::{parse_outcome::parse_outcome, parse_setup_step::parse_setup_step};
use crate::Outcome;
use syn::{
  ExprCall, LitStr, Result,
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
        behavior.outcomes_mut().push(parse_outcome(input)?);
      } else {
        behavior.setup_steps_mut().push(parse_setup_step(input)?);
      }
    }

    Ok(behavior)
  }
}
