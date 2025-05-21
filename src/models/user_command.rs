use super::utils::{Datetime, Email, Phone};

pub struct Password(String);

pub struct UserId(pub i32);

pub struct NewUser {
  pub full_name: String,
  pub phone: Phone,
  pub birth_date: Option<Datetime>,
}

pub struct User {
  pub user_id: UserId,
  pub full_name: String,
  pub phone: Phone,
  pub birth_date: Option<Datetime>,
}

pub struct Employee {
  pub phone: Option<Phone>,
  pub department: String,
  pub position: String,
}

pub struct AccountId(pub i32);

pub struct Account {
  pub user_id: UserId,
  pub email: Email,
  pub password: Password,
}
