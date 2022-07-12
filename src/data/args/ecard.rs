use clap::Args;

#[derive(Args)]
pub struct EcardArg {
  /// Show details
  #[clap(short, long)]
  pub detail: bool,
}
