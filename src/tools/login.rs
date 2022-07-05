use anyhow::{Result, bail};
use regex::Regex;
use reqwest::Client;
use crate::data::{AUTH_URL, Storage, STORAGE_FILE, Config};

pub fn get_info(
  config: &Config,
  username: Option<String>,
  password: Option<String>
) -> Option<(String, String)> {
  let no_username = if let Some(basic) = &config.basic {
    basic.username.is_none()
  } else {
    true
  };
  let no_password = if let Some(basic) = &config.basic {
    basic.password.is_none()
  } else {
    true
  };
  if username.is_none() && no_username {
    None
  } else if password.is_none() && no_password {
    None
  } else {
    let u = username.unwrap_or_else(|| config.basic.as_ref().unwrap().username.as_ref().unwrap().clone());
    let p = password.unwrap_or_else(|| config.basic.as_ref().unwrap().password.as_ref().unwrap().clone());
    Some((u, p))
  }
}

pub async fn login(
  storage: &mut Storage,
  username: &str,
  password: &str
) -> Result<()> {
  let client = Client::new();

  let re = Regex::new("<input type=\"hidden\" name=\"execution\" value=\"(.+?)\"/>").unwrap();
  let execution_raw = client.get(AUTH_URL).send().await?.text().await?;
  let execution = re.captures(&execution_raw).unwrap().get(1).unwrap().as_str();

  let login_resp = client.post(AUTH_URL)
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
