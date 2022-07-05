use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BasicConfig {
  pub username: Option<String>,
  pub password: Option<String>,
}
