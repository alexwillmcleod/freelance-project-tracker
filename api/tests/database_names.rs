#[tokio::main]
async fn main() {
  let body = reqwest::get("http://127.0.0.1:3000/database")
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  println!("Body: {:#?}", body);
}
