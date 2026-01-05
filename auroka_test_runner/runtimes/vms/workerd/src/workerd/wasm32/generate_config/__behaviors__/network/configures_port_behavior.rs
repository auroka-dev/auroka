use super::super::super::__steps__::Context;
use super::super::super::__steps__::given_the_port_is;
use super::super::super::__steps__::then_the_config_should_contain;
use super::super::super::__steps__::when_the_config_is_generated;

#[test]
pub fn configures_port_behavior() {
  let mut context = Context::new();

  given_the_port_is(&mut context, 9090);

  when_the_config_is_generated(&mut context);

  then_the_config_should_contain(&mut context, r#"address = "*:9090""#);
}
