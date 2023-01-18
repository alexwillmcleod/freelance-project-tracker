use axum::{extract::State, response::IntoResponse};

use crate::AppState;

pub async fn create_project(State(app_state): State<AppState>) -> impl IntoResponse {

  // Connect to our database and create a project
  // Extract from JSON, project name, link to user creating it.
}
