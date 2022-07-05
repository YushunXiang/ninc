use clap::Args;

#[derive(Args)]
pub struct EsrepArg {
  /// Always yes
  #[clap(short)]
  pub yes: bool,
}
