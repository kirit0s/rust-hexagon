use super::{
  user_command::{AccountId, UserId},
  utils::{Datetime, Email, Password, Phone},
};

pub struct User {
  pub user_id: UserId,
  pub full_name: String,
  pub phone: Phone,
  pub birth_date: Option<Datetime>,
  pub created_at: Datetime,
}

pub struct Account {
  pub account_id: AccountId,
  pub user_id: UserId,
  pub email: Email,
  pub password: Password,
  pub created_at: Datetime,
}
