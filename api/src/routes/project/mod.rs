mod create_project;
mod get_project_list;
pub mod todo_list;

use todo_list::Todo;

pub use create_project::create_project;
pub use get_project_list::get_project_list;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct NewProject {
  pub user: mongodb::bson::oid::ObjectId,
  pub name: String,
  pub todo_list: Vec<Todo>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Project {
  pub _id: mongodb::bson::oid::ObjectId,
  pub user: mongodb::bson::oid::ObjectId,
  pub name: String,
  pub todo_list: Vec<Todo>,
}
