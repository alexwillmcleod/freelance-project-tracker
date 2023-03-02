use crate::LoggedInUser;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn get_user(
  Extension(option_user): Extension<Option<LoggedInUser>>,
) -> impl IntoResponse {
  (StatusCode::OK, Json(option_user))
}
