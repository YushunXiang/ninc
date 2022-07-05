use clap::Args;

#[derive(Args)]
pub struct LoginArg {
  /// Your username: student number, phone number, email address or identity card number
  #[clap(short)]
  pub username: Option<String>,
  
  /// Your password
  #[clap(short)]
  pub password: Option<String>,

  /// Save your username and password to config file
  #[clap(long)]
  pub save: bool,
}
