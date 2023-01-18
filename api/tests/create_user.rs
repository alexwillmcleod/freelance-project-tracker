use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
  let mut map = HashMap::new();
  map.insert("email", "zacdavidmcleod@gmail.com");
  map.insert("first_name", "Alex");
  map.insert("last_name", "McLeod");
  map.insert("password", "password123");

  let client = reqwest::Client::new();
  let body = client
    .post("http://127.0.0.1:3000/user")
    .json(&map)
    .send()
    .await
    .unwrap();

  #[derive(Debug, Serialize, Deserialize)]
  struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
  }

  let created_user: User = serde_json::from_str(body.text().await.unwrap().as_str()).unwrap();

  println!("{:#?}", created_user);
}
