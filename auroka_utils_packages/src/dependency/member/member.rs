#[derive(Debug)]
pub struct Member {
  name: String,
  member: String,
}

impl Member {
  pub fn new(name: &str, member: &str) -> Self {
    Member { name: name.to_string(), member: member.to_string() }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn member(&self) -> &str {
    &self.member
  }
}
