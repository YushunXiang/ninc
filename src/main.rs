mod commands;

use clap::{Parser, Subcommand};
use commands::login;

#[derive(Parser)]
#[clap(name = "ninc")]
#[clap(about = "A command line interface for NWPU ecampus", long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Sign in to your account
  #[clap(arg_required_else_help = true)]
  Login {
    #[clap(short)]
    username: String,
    
    #[clap(short)]
    password: String,
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();

  match args.command {
    Commands::Login { username, password } => {
      if let Err(err) = login(&username, &password).await {
        eprintln!("{}", err)
      }
    }
  }

  Ok(())
}
