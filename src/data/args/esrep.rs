use clap::Args;

#[derive(Args)]
pub struct EsrepArg {
  /// Always yes
  #[clap(short)]
  pub yes: bool,

  /// Whether in school
  #[clap(short, long)]
  pub school: bool,

  /// Address code
  #[clap(short = 'C', long)]
  pub code: Option<i32>,

  /// Address
  #[clap(short, long)]
  pub address: Option<String>,

  /// Phone number
  #[clap(short, long)]
  pub phone: Option<String>,

  /// College name
  #[clap(short, long)]
  pub college: Option<String>,
}
