use anyhow::{bail, Result};
use crate::{
  data::{
    Config,
    CONFIG_FILE,
    Storage
  },
  tools::{self, auth::home}
};

pub async fn login(
  config: &mut Config,
  storage: &mut Storage,
  username: &str,
  password: &str,
  save: bool
) -> Result<()> {
  if let Err(err) = tools::login(storage, username, password).await {
    bail!("{}", err);
  }

  if save {
    config.basic.username = Some(String::from(username));
    config.basic.password = Some(String::from(password));
    if let Err(err) = config.save(CONFIG_FILE).await {
      eprintln!("Failed to save config file!\n{}", err);
    }
  }

  if let Err(err) = home::auth(storage).await {
    bail!("Auth failed: {}", err);
  }

  Ok(())
}
