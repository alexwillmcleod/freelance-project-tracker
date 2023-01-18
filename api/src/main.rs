use axum::{
  extract::State,
  middleware,
  routing::{get, post},
  Extension, Router,
};
use mongodb::{
  options::{ClientOptions, ResolverConfig},
  Client,
};

use dotenv::dotenv;
use freelance_api::{
  middleware::auth,
  routes::{
    project::create_project,
    user::{create_user, login_user},
  },
  AppState, LoggedInUser, User,
};

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // initialize tracing
  tracing_subscriber::fmt::init();

  // Load in .env file
  dotenv().ok();

  // Load mongodb connection string from environment variable
  let mongo_client_uri = match env::var("MONGODB_URI") {
    Ok(uri) => uri,
    Err(..) => panic!("MONGODB_URI environment variable is not set"),
  };

  // Create a client
  let options =
    ClientOptions::parse_with_resolver_config(&mongo_client_uri, ResolverConfig::cloudflare())
      .await?;

  let mongo_client = Client::with_options(options)?;

  let app_state = AppState { mongo_client };

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    // `POST /users` goes to `create_user`
    .route("/user/create", post(create_user))
    .route("/user/login", get(login_user))
    .route("/project/create", post(create_project))
    .layer(
      ServiceBuilder::new()
        .layer(Extension::<Option<LoggedInUser>>(None))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth)),
    )
    .layer(CookieManagerLayer::new())
    .with_state(app_state);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}

/*
GET /
Returns "Hello, World!" for non logged in users and
"Hello, Joe Blogs!" for logged in users
*/
async fn root(Extension(option_user): Extension<Option<LoggedInUser>>) -> String {
  let Some(user) = option_user else {
    return String::from("Hello, World")
  };
  format!("Hello, {} {}!", user.first_name, user.last_name)
}
