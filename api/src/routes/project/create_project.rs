use crate::{LoggedInUser, NewProject, Project};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use bson::doc;
use mongodb::Collection;
use reqwest::StatusCode;

use crate::AppState;

pub async fn create_project(
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
  match project_database
    .find_one(
      doc! {
        "user": &user._id,
        "name": &project.name
      },
      None,
    )
    .await
    .expect("Database search did not work")
  {
    Some(_) => {
      return Err((
        StatusCode::BAD_REQUEST,
        "Project with that name already exists",
      ))
    }
    None => {}
  }

  // Instantiate Project object
  let project = Project {
    user: user._id,
    name: project.name,
  };

  // Add project to database
  project_database.insert_one(project, None).await.unwrap();

  Ok((StatusCode::OK, "Successfully created project"))
}
