use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginStorage {
  pub cookie_tgc: Option<String>,
  pub cookie_jwt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
  pub login: LoginStorage,
}

impl Storage {
  pub async fn load(path: &str) -> Result<Self> {
    let storage_str = tokio::fs::read_to_string(path).await?;
    let storage = serde_json::from_str(&storage_str)?;
    Ok(storage)
  }
  pub async fn save(&self, path: &str) -> Result<()> {
    let storage_str = serde_json::to_string(&self)?;
    tokio::fs::write(path, storage_str).await?;
    Ok(())
  }
  pub fn new() -> Self {
    Storage {
      login: LoginStorage {
        cookie_tgc: None,
        cookie_jwt: None
      }
    }
  }
}
