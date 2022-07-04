use anyhow::{Result, bail};
use regex::Regex;
use crate::data::{LOGIN_URL, Config, CONFIG_FILE, Storage, STORAGE_FILE};

pub async fn login(
  config: &mut Config,
  storage: &mut Storage,
  username: &str,
  password: &str,
  save: bool
) -> Result<()> {
  let client = reqwest::Client::new();

  // let pubkey = client.get("https://uis.nwpu.edu.cn/cas/jwt/publicKey").send().await?.text().await?;

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
    bail!("Sign in failed!\nPlease check your username and password.");
  }

  let cookie_raw = login_resp.headers().get("set-cookie").unwrap().to_str().unwrap();
  let cookie_tgc = &cookie_raw.split(';').next().unwrap()[4..];

  if save {
    config.basic.username = String::from(username);
    config.basic.password = String::from(password);
    if let Err(err) = config.save(CONFIG_FILE).await {
      eprintln!("Failed to save config file!\n{}", err);
    }
  }

  storage.login.cookie_tgc = String::from(cookie_tgc);
  if let Err(err) = storage.save(STORAGE_FILE).await {
    eprintln!("Failed to save storage file!\n{}", err);
  }

  Ok(())
}
