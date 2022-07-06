#![allow(non_snake_case, dead_code)]

use anyhow::{Result, bail};
use serde::Deserialize;
use reqwest::Client;
use crate::data::{User, Storage, USER_URL, STORAGE_FILE};

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

pub async fn get_user(storage: &mut Storage) -> Result<User> {
  let client = Client::new();

  if let Some(jwt) = &storage.login.cookie_jwt {
    let resp = client.get(USER_URL).header("x-id-token", jwt).send().await?;
    let data: Resp = resp.json().await?;
    let uid = data.data.username;
    let name = data.data.attributes.userName;
    storage.basic.uid = Some(uid.clone());
    storage.basic.name = Some(name.clone());
    storage.save(STORAGE_FILE).await?;
    Ok(User {
      identity: data.data.attributes.identityTypeName,
      uid,
      name,
    })
  } else {
    bail!("Please sign in first!");
  }
}
