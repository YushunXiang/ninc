use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginStorage {
  pub cookie_tgc: Option<String>,
  pub cookie_jwt: Option<String>,
}
