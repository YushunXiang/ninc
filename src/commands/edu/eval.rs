use anyhow::Result;
use reqwest::Client;
use crate::data::{args::edu::eval::EvalCommands, EDU_EVAL_SEME};

async fn seme() -> Result<()> {
  let client = Client::new();

  let resp = client.get(EDU_EVAL_SEME);
  Ok(())
}

pub async fn eval(cmd: EvalCommands) -> Result<()> {
  match cmd {
    EvalCommands::Seme => {
      Ok(())
    }
    EvalCommands::List(args) => {
      Ok(())
    }
  }
}
