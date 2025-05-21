use crate::models::user_command;

pub trait UserCommand {
  fn create_user(user: user_command::User) {}
  fn update_user(user: user_command::User) {}
}

pub trait UserQuery {
  fn x() {}
}
