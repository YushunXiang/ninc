mod login;
mod esrep;

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
  #[clap(arg_required_else_help = true)]
  Login(login::LoginArg),

  /// Epidemic situation report
  Esrep(esrep::EsrepArg),
}
