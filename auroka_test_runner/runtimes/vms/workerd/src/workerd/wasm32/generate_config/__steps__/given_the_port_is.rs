use super::Context;

pub fn given_the_port_is(context: &mut Context, port: u16) {
  context.port_set(port);
}
