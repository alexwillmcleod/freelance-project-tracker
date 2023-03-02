pub mod middleware;
pub mod routes;

pub use routes::project::{NewProject, Project};
pub use routes::user::{LoggedInUser, LoginUser, NewUser, User};

use serde::{Deserialize, Serialize};

pub const WRONG_LOGIN_DETAILS_ERROR_MSG: &'static str = "Email or password is incorrect";

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SessionToken {
  pub email: String,
}

#[derive(Clone)]
pub struct AppState {
  pub mongo_client: mongodb::Client,
  pub aws_s3_client: aws_sdk_s3control::Client,
}
