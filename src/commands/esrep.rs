use anyhow::Result;
use reqwest::Client;
use crate::{data::{Storage, ESREP_URL}, tools::auth::esrep};

pub async fn esrep(storage: &Storage) -> Result<()> {
  let jsessionid = esrep::auth(&storage).await?;

  // TODO: add functions related to report system

  let client = Client::new();

  let resp = client.get(ESREP_URL)
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .send().await?;

  println!("{}", resp.text().await?);

  Ok(())
}
