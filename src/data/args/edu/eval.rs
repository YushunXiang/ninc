use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ListArgs {

}

#[derive(Subcommand)]
pub enum EvalCommands {
  /// Get enable semesters
  Seme,

  /// Get the teaching evaluation list
  List(ListArgs),
}
