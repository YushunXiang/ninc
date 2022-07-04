use anyhow::{Result, bail};
use regex::Regex;
use reqwest::Client;
use crate::data::{LOGIN_URL, Storage, STORAGE_FILE};

#[allow(dead_code)]
pub async fn login(
  storage: &mut Storage,
  username: &str,
  password: &str
) -> Result<()> {
  let client = Client::new();

  let re = Regex::new("<input type=\"hidden\" name=\"execution\" value=\"(.+?)\"/>").unwrap();
  let execution_raw = client.get(LOGIN_URL).send().await?.text().await?;
  let execution = re.captures(&execution_raw).unwrap().get(1).unwrap().as_str();

  let login_resp = client.post(LOGIN_URL)
    .form(&[
      ("username", username),
      ("password", password),
      ("execution", execution),
      ("_eventId", "submit"),
    ]).send().await?;

  if login_resp.status() == 401 {
    bail!("Please check your username and password.");
  }

  let tgc_raw = login_resp.headers().get("set-cookie").unwrap().to_str().unwrap();
  let tgc = &tgc_raw.split(';').next().unwrap()[4..];

  storage.login.cookie_tgc = Some(String::from(tgc));
  storage.save(STORAGE_FILE).await?;

  Ok(())
}

pub async fn login_with_client(
  storage: &mut Storage,
  username: &str,
  password: &str,
  client: &Client
) -> Result<()> {
  let re = Regex::new("<input type=\"hidden\" name=\"execution\" value=\"(.+?)\"/>").unwrap();
  let execution_raw = client.get(LOGIN_URL).send().await?.text().await?;
  let execution = re.captures(&execution_raw).unwrap().get(1).unwrap().as_str();

  let login_resp = client.post(LOGIN_URL)
    .form(&[
      ("username", username),
      ("password", password),
      ("execution", execution),
      ("_eventId", "submit"),
    ]).send().await?;

  if login_resp.status() == 401 {
    bail!("Please check your username and password.");
  }

  let tgc_raw = login_resp.headers().get("set-cookie").unwrap().to_str().unwrap();
  let tgc = &tgc_raw.split(';').next().unwrap()[4..];

  storage.login.cookie_tgc = Some(String::from(tgc));
  storage.save(STORAGE_FILE).await?;

  Ok(())
}
