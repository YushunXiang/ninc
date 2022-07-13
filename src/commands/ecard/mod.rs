#![allow(non_snake_case, dead_code)]
mod short;
mod long;

use anyhow::{bail, Result};
use serde::Deserialize;
use reqwest::Client;
use crate::data::{Config, Storage};
use self::{short::ecard_short, long::ecard_long};

#[derive(Deserialize)]
struct Resp<T> {
  code: i32,
  data: Option<T>,
  message: Option<String>,
}

pub async fn ecard(
  _config: &Config,
  storage: &Storage,
  is_detail: bool,
) -> Result<()> {
  let client = Client::new();

  let jwt = if let Some(jwt) = &storage.login.cookie_jwt {
    jwt
  } else {
    bail!("Please sign in first!");
  };

  if is_detail {
    ecard_long(jwt, &client).await?;
  } else {
    ecard_short(jwt, &client).await?;
  }
  Ok(())
}
