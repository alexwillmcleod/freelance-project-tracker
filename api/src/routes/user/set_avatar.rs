use crate::{AppState, LoggedInUser};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn set_avatar(
  State(app_state): State<AppState>,
  Extension(option_user): Extension<Option<LoggedInUser>>,
) -> impl IntoResponse {
  // We want to connect to the database
  // Upload our new file
  // Change the address of our users profile picture to the new file
  todo!();
}
