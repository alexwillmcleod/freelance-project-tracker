use axum::{
  extract::State,
  http::{Request, StatusCode},
  middleware::Next,
  response::IntoResponse,
};
use bson::doc;
use mongodb::Collection;

use crate::WRONG_LOGIN_DETAILS_ERROR_MSG;
use crate::{AppState, LoggedInUser, SessionToken, User};
use jwt_compact::{
  alg::{Hs256, Hs256Key},
  prelude::*,
};
use std::env;
use tower_cookies::Cookies;

pub async fn auth<B>(
  State(app_state): State<AppState>,
  cookies: Cookies,
  mut request: Request<B>,
  next: Next<B>,
) -> impl IntoResponse {
  let Some(session_cookie) = cookies.get("session-token") else {
    let response = next.run(request).await;
    return Ok(response);
  };

  let token_string: &str = session_cookie.value();

  let token = UntrustedToken::new(&token_string).unwrap();
  assert_eq!(token.header().key_id.as_deref(), Some("session-token"));

  let jwt_secret = env::var("JWT_SECRET").expect("Must have JWT_SECRET environment variable set");
  let time_options = TimeOptions::default();
  let key = Hs256Key::new(jwt_secret.as_bytes());

  let token: Token<SessionToken> = Hs256.validate_integrity(&token, &key).unwrap();
  let session = token
    .claims()
    .validate_expiration(&time_options)
    .unwrap()
    .validate_maturity(&time_options)
    .unwrap();

  let email = &session.custom.email;

  // Get the user information from the database
  let user_database: Collection<User> = app_state
    .mongo_client
    .database("freelance")
    .collection("users");

  let Some(user) = user_database
    .find_one(
      doc! {
        "email": email
      },
      None,
    )
    .await
    .expect("Database search failed") else {
      return Err((StatusCode::BAD_REQUEST, WRONG_LOGIN_DETAILS_ERROR_MSG))
    };

  let logged_in_user = Some(LoggedInUser {
    email: user.email,
    first_name: user.first_name,
    last_name: user.last_name,
  });

  request.extensions_mut().insert(logged_in_user);

  let response = next.run(request).await;
  Ok(response)
}
