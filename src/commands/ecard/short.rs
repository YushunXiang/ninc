use anyhow::{bail, Result};
use serde::Deserialize;
use reqwest::Client;
use crate::data::ECARD_URL;
use super::Resp;

#[derive(Deserialize)]
struct EcardLastCas {
  address: String,
  time: String,
}

#[derive(Deserialize)]
struct EcardExpenditure {
  address: String,
  amount: String,
  payWay: String,
  time: String,
}

#[derive(Deserialize)]
struct EcardShort {
  appUrl: String,
  balance: String,
  cardStatus: String,
  eCardType: String,
  lastCas: EcardLastCas,
  lastExpenditure: Vec<EcardExpenditure>,
  lastIncome: Vec<i32>,
  monthBalance: String,
  openDate: String,
  pcUrl: String,
}

pub async fn ecard_short(jwt: &str, client: &Client) -> Result<()> {
  let resp = client.get(ECARD_URL).header("x-id-token", jwt).send().await?;

  let data: Resp<EcardShort> = resp.json().await?;
  
  if data.code == 0 {
    let data = data.data.unwrap();
    println!("Status:  {}", data.cardStatus);
    println!("Balance: {} CNY", data.balance);
    println!("{} CNY has been expended this month.", data.monthBalance);
    println!("\nRecent expenditure:");
    for item in data.lastExpenditure {
      println!("{:3} {:7} {}", item.payWay, item.amount, item.time);
    }
    Ok(())
  } else {
    bail!("[{}] {}", data.code, data.message.unwrap())
  }
}
