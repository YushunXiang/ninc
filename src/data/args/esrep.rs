use clap::Args;

#[derive(Args)]
pub struct EsrepArg {
  /// Get information of the report
  #[clap(long)]
  pub get: bool,
}
