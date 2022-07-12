mod login;
mod esrep;
mod ecard;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "ninc", author = "yurzhang")]
#[clap(about = "A command line interface for NWPU ecampus", long_about = None)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Sign in to your account
  Login(login::LoginArg),

  /// Epidemic situation report
  Esrep(esrep::EsrepArg),

  /// Query consumption records
  Ecard(ecard::EcardArg),
}
