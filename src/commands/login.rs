use anyhow::{bail, Result};
use reqwest::Client;
use crate::{
  data::{
    Config,
    CONFIG_FILE,
    Storage,
    BasicConfig
  },
  tools
};

pub async fn login(
  config: &mut Config,
  storage: &mut Storage,
  username: &str,
  password: &str,
  save: bool
) -> Result<()> {
  let client = Client::new();

  if let Err(err) = tools::login_with_client(storage, username, password, &client).await {
    bail!("Sign in failed!\n{}", err);
  }

  if save {
    config.basic = Some(BasicConfig {
      username: String::from(username),
      password: String::from(password)
    });
    if let Err(err) = config.save(CONFIG_FILE).await {
      eprintln!("Failed to save config file!\n{}", err);
    }
  }

  if let Err(err) = tools::auth(storage).await {
    bail!("Auth failed!\n{}", err);
  }

  Ok(())
}
