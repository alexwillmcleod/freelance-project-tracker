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

use super::Todo;

pub async fn update_todo_list(
  State(app_state): State<AppState>,
  Extension(option_user): Extension<Option<LoggedInUser>>,
  Path(project_name): Path<String>,
  Json(todo_list): Json<Vec<Todo>>,
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
  let Some(mut project): Option<Project> = project_database
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

  project.todo_list = todo_list;

  project_database
    .find_one_and_replace(
      doc! {
        "name": project_name,
        "user": &user._id

      },
      &project,
      None,
    )
    .await
    .unwrap();

  Ok((StatusCode::OK, Json(project)))
}
