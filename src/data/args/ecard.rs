use clap::Args;

#[derive(Args)]
pub struct EcardArg {
  /// Show details
  #[clap(short, long)]
  pub detail: bool,

  /// Begin date (YYYY-MM-DD)
  #[clap(short, long)]
  pub begin: Option<String>,

  /// End date (YYYY-MM-DD)
  #[clap(short, long)]
  pub end: Option<String>,

  /// Maximum display quantity
  #[clap(short, long)]
  pub limit: Option<usize>,
}
