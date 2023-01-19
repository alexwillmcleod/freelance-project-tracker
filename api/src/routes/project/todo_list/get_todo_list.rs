use crate::{LoggedInUser, Project};
use axum::{
  extract::{Path, State},
  response::IntoResponse,
  Extension, Json,
};
use bson::doc;
use mongodb::Collection;
use reqwest::StatusCode;

use crate::AppState;

pub async fn get_todo_list(
  State(app_state): State<AppState>,
  Extension(option_user): Extension<Option<LoggedInUser>>,
  Path(project_name): Path<String>,
) -> impl IntoResponse {
  // Confirm that the user is logged in
  let Some(user) = option_user else {
    return Err((StatusCode::UNAUTHORIZED, String::from("You must be signed in to update todos")));
  };

  // Connect to our database
  let project_database: Collection<Project> = app_state
    .mongo_client
    .database("freelance")
    .collection("projects");

  // Try finding project
  let Some(project): Option<Project> = project_database
    .find_one(
      doc! {
        "user": &user._id,
        "name": &project_name
      },
      None,
    )
    .await
    .unwrap() else {
      return Err((StatusCode::BAD_REQUEST, format!("Could not find project {}", project_name)));
    };

  Ok((StatusCode::OK, Json(project.todo_list)))
}
