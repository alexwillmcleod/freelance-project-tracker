mod get_todo_list;
mod update_todo_list;

use bson::doc;

pub use get_todo_list::get_todo_list;
pub use update_todo_list::update_todo_list;

use mongodb::bson::DateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
  pub name: String,
  pub description: Option<String>,
  pub due_date: Option<DateTime>,
  pub cost: Option<f64>,
}
