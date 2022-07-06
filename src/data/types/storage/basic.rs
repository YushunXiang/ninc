use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BasicStorage {
  pub uid: Option<String>,
  pub name: Option<String>,
}
