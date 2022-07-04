#![allow(non_snake_case, dead_code)]

use anyhow::{Result, bail};
use serde::Deserialize;
use reqwest::Client;
use crate::data::{User, Storage, USER_URL};

#[derive(Deserialize)]
struct RespAttr {
  accountId: String,
  identityTypeCode: String,
  identityTypeId: String,
  identityTypeName: String,
  imageUrl: String,
  organizationCode: String,
  organizationId: String,
  organizationName: String,
  userId: String,
  userName: String,
  userUid: String,
}

#[derive(Deserialize)]
struct RespData {
  attributes: RespAttr,
  roles: Vec<String>,
  username: String,
}

#[derive(Deserialize)]
struct Resp {
  acknowleged: bool,
  code: i32,
  data: RespData,
  message: Option<String>,
}

pub async fn get_user(storage: &Storage) -> Result<User> {
  let client = Client::new();

  if let Some(jwt) = &storage.login.cookie_jwt {
    let resp = client.get(USER_URL).header("x-id-token", jwt).send().await?;
    let data: Resp = resp.json().await?;
    Ok(User {
      username: data.data.username,
      name: data.data.attributes.userName
    })
  } else {
    bail!("Please sign in first!");
  }
}
