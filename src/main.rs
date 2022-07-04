mod data;
mod commands;

use clap::Parser;
use data::{
  Config,
  Storage,
  CONFIG_FILE,
  STORAGE_FILE,
  args::{Cli, Commands}
};
use commands::login;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();

  let mut config = match Config::load(CONFIG_FILE).await {
    Ok(config) => { config },
    Err(err) => {
      eprintln!("Failed to load config file!\n{}", err);
      Config::new()
    }
  };

  let mut storage = match Storage::load(STORAGE_FILE).await {
    Ok(storage) => { storage },
    Err(err) => {
      eprintln!("Failed to load storage file!\n{}", err);
      Storage::new()
    }
  };

  match args.command {
    Commands::Login(args) => {
      if let Err(err) = login(&mut config, &mut storage, &args.username, &args.password, args.save).await {
        eprintln!("{}", err)
      }
    }
  }

  Ok(())
}
