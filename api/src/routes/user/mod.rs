mod create_user;
mod login_user;

pub use create_user::create_user;
pub use login_user::login_user;

use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub _id: mongodb::bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggedInUser {
  pub _id: mongodb::bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewUser {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
}
