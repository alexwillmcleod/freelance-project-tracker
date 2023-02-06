use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::{bson::doc, Collection};

use crate::WRONG_LOGIN_DETAILS_ERROR_MSG;
use crate::{AppState, LoginUser, SessionToken, User};
use bcrypt::verify;
use chrono::{Duration, Utc};
use cookie::{Cookie, SameSite};
use jwt_compact::{
  alg::{Hs256, Hs256Key},
  prelude::*,
};
use std::env;
use tower_cookies::Cookies;

pub async fn login_user(
  cookies: Cookies,
  State(app_state): State<AppState>,
  Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
  let user_database: Collection<User> = app_state
    .mongo_client
    .database("freelance")
    .collection("users");

  // Get the user with that email
  let Some(user) = user_database
    .find_one(
      doc! {
        "email": &payload.email
      },
      None,
    )
    .await
    .expect("Database search failed") else {
      return Err((StatusCode::BAD_REQUEST, String::from(WRONG_LOGIN_DETAILS_ERROR_MSG)))
    };

  // Use bcrypt library to check if password is valid
  let is_password_valid = verify(payload.password, &user.password).unwrap();

  // If password is not valid return error message
  if !is_password_valid {
    return Err((
      StatusCode::BAD_REQUEST,
      String::from(WRONG_LOGIN_DETAILS_ERROR_MSG),
    ));
  }

  // Password is therefore valid
  // Issue a JWT token
  let jwt_secret = env::var("JWT_SECRET").expect("Must have JWT_SECRET environment variable set");

  let time_options = TimeOptions::default();
  let key = Hs256Key::new(jwt_secret.as_bytes());

  let claims = Claims::new(SessionToken {
    email: String::from(&user.email),
  })
  .set_duration_and_issuance(&time_options, Duration::weeks(2))
  .set_not_before(Utc::now() - Duration::hours(1));

  let header = Header::default().with_key_id("session-token");

  let token = Hs256
    .token(header, &claims, &key)
    .expect("Could not create token");

  let cookie_domain =
    std::env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN environment variable must be set");

  cookies.add(
    Cookie::build("session-token", token)
      // .secure(true)
      .path("/")
      // .domain(cookie_domain)
      .same_site(SameSite::None)
      .finish(),
  );

  Ok((StatusCode::OK, String::from("Successfully logged in")))
}
