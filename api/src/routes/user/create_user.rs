use crate::{AppState, User};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::{bson::doc, Collection};

use bcrypt::{hash, DEFAULT_COST};

pub async fn create_user(
  // this argument tells axum to parse the request body
  // as JSON into a `User` type
  State(app_state): State<AppState>,
  Json(payload): Json<User>,
) -> impl IntoResponse {
  // Get user database
  let user_database: Collection<User> = app_state
    .mongo_client
    .database("freelance")
    .collection("users");

  // Check that email is not taken
  if let Some(_) = user_database
    .find_one(
      doc! {
        "email": &payload.email
      },
      None,
    )
    .await
    .expect("Database search failed")
  {
    return Err((
      StatusCode::BAD_REQUEST,
      String::from("User with that email already exists"),
    ));
  }

  // Hash password
  let hashed_password = hash(payload.password, DEFAULT_COST).expect("Could not hash password");

  // Create user
  let user = User {
    email: payload.email,
    first_name: payload.first_name,
    last_name: payload.last_name,
    password: hashed_password,
  };
  // Insert user into database
  user_database.insert_one(&user, None).await.unwrap();

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  Ok((
    StatusCode::CREATED,
    format!("Created account with email {}", user.email),
  ))
}
