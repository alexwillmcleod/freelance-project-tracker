use axum::{
  http::HeaderValue,
  middleware,
  routing::{get, patch, post},
  Extension, Router,
};
use mongodb::{options::ClientOptions, Client};
use tower_http::cors::{Any, CorsLayer};

use dotenv::dotenv;
use freelance_api::{
  middleware::auth,
  routes::{
    project::{
      create_project, get_project_list,
      todo_list::{get_todo_list, update_todo_list},
    },
    user::{create_user, get_user, login_user},
  },
  AppState, LoggedInUser,
};

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use log::{debug, info};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Load in .env file
  dotenv().ok();

  // Initialise Logging
  env_logger::init();
  info!("Starting up!");
  println!("Starting app up!");

  // Load mongodb connection string from environment variable
  let mongo_client_uri = match env::var("MONGODB_URI") {
    Ok(uri) => uri,
    Err(..) => panic!("MONGODB_URI environment variable is not set"),
  };

  println!("Creating client options");
  println!("mongo_client_uri = {}", mongo_client_uri);
  // Create a client
  // let options =
  //   ClientOptions::parse_with_resolver_config(&mongo_client_uri, ResolverConfig::cloudflare())
  //     .await?;

  let options = ClientOptions::parse(mongo_client_uri)
    .await
    .expect("Could not create client options");

  println!("Connecting to mongo db!");
  let mongo_client = Client::with_options(options)?;
  println!("Connected to mongo db!");

  let aws_config = aws_config::load_from_env().await;
  let aws_s3_client = aws_sdk_s3control::Client::new(&aws_config);

  let app_state = AppState {
    mongo_client,
    aws_s3_client,
  };

  // Build app with router
  let app = Router::new()
    .route("/", get(root))
    .route("/user", get(get_user))
    .route("/user/create", post(create_user))
    .route("/user/login", post(login_user))
    .route("/project/create/:project_name", post(create_project))
    .route("/project/list", get(get_project_list))
    .route("/project/todos/:project_name", patch(update_todo_list))
    .route("/project/todos/:project_name", get(get_todo_list))
    .layer(
      ServiceBuilder::new()
        .layer(Extension::<Option<LoggedInUser>>(None))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth)),
    )
    .layer(
      CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .expose_headers(Any),
    )
    .layer(CookieManagerLayer::new())
    .with_state(app_state);

  // Get port from environment variables
  let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| 3000.to_string())
    .parse()
    .unwrap();

  println!("Using port {}", port);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([0, 0, 0, 0], port));
  info!("Serving at {}", addr);
  println!("Serving at {}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}

/*
  GET /
  Returns "Hello, Anonymous User!" for non logged in users and
  "Hello, Joe Blogs!" for logged in users as well as their ID
*/
async fn root(Extension(option_user): Extension<Option<LoggedInUser>>) -> String {
  debug!("/GET /");
  let Some(user) = option_user else {
    return String::from("Hello, Anonymous User!")
  };
  format!(
    "Hello, {} {}! Your ID is {}",
    user.first_name, user.last_name, user._id
  )
}
