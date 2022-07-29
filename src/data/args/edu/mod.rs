pub mod eval;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum EduCommands {
  /// Functions related to teaching evaluation
  #[clap(subcommand)]
  Eval(eval::EvalCommands),
}
