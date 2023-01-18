use crate::{LoggedInUser, NewProject, Project};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use bson::doc;
use futures::{StreamExt, TryStreamExt};
use mongodb::Collection;
use reqwest::StatusCode;

use crate::AppState;

pub async fn get_project_list(
  Extension(option_user): Extension<Option<LoggedInUser>>,
  State(app_state): State<AppState>,
  Json(project): Json<NewProject>,
) -> impl IntoResponse {
  // Connect to our database and create a project
  // Extract from JSON, project name, link to user creating it.
  let project_database: Collection<Project> = app_state
    .mongo_client
    .database("freelance")
    .collection("projects");

  // Check that we are logged in
  let Some(user) = option_user else {
    return Err((StatusCode::UNAUTHORIZED, "You must be logged in to create a project"));
  };

  // Check that no project with that name already exists
  let project_name_list: Vec<String> = match project_database
    .find(
      doc! {
        "user": &user._id,
      },
      None,
    )
    .await
  {
    Ok(cursor) => cursor.try_collect().await.unwrap_or_else(|_| vec![]),
    Err(..) => vec![],
  }
  .iter_mut()
  .map(|x| x.name.clone())
  .collect();

  Ok((StatusCode::OK, Json(project_name_list)))
}
