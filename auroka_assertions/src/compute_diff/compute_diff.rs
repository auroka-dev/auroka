extern crate diff;

pub fn compute_diff(left: &str, right: &str) -> String {
  let mut buffer = String::new();

  for diff in diff::lines(left, right) {
    match diff {
      diff::Result::Left(l) => buffer += &format!("-{l}\n"),
      diff::Result::Both(l, _) => buffer += &format!(" {l}\n"),
      diff::Result::Right(r) => buffer += &format!("+{r}\n"),
    }
  }

  buffer
}
