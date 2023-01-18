pub mod middleware;
pub mod routes;

use mongodb::Client;
use serde::{Deserialize, Serialize};

pub const WRONG_LOGIN_DETAILS_ERROR_MSG: &'static str = "Email or password is incorrect";

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggedInUser {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SessionToken {
  pub email: String,
}

#[derive(Clone)]
pub struct AppState {
  pub mongo_client: Client,
}
