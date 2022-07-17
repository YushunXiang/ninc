use clap::Args;

// TODO: set args for edu

#[derive(Args)]
pub struct EduArg {
  /// Always yes
  #[clap(short)]
  pub yes: bool,
}
