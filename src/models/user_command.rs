use super::utils::{Datetime, Email, Password, Phone};

pub struct UserId(pub i32);

pub struct User {
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
  pub salt: String,
}
