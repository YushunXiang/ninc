#![allow(non_snake_case, dead_code)]
use anyhow::{bail, Result};
use serde::Deserialize;
use chrono::{Local, Duration};
use reqwest::Client;
use crate::data::{Config, Storage, ECARD_URL, ECARD_DETAIL_URL};

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

#[derive(Deserialize)]
struct Resp<T> {
  code: i32,
  data: T,
  message: Option<String>,
}

pub async fn ecard_short(jwt: &str, client: &Client) -> Result<()> {
  let resp = client.get(ECARD_URL).header("x-id-token", jwt).send().await?;
  let data: Resp<EcardShort> = resp.json().await?;
  
  if data.code == 0 {
    let data = data.data;
    println!("Status:  {}", data.cardStatus);
    println!("Balance: {} CNY", data.balance);
    println!("{} CNY has been expended this month.", data.monthBalance);
    println!("\nRecent expenditure:");
    for item in data.lastExpenditure {
      println!("{:3} {:7} {}", item.payWay, item.amount, item.time);
    }
    Ok(())
  } else {
    bail!(data.message.unwrap())
  }
}

#[derive(Deserialize)]
struct EcardOperation {
  blance: f64, // What the fxxk?
  occurrenceTime: String,
  operationTitle: String,
  operationType: String,
  payment: String,
}

#[derive(Deserialize)]
struct EcardDetail {
  consume: String,
  costList: Vec<EcardOperation>,
  totalItems: i32,
  totalSum: i32,
}

#[derive(Deserialize)]
struct EcardLong {
  data: EcardDetail
}

pub async fn ecard_long(jwt: &str, client: &Client) -> Result<()> {
  let now = Local::now();
  let begin = now - Duration::days(30);

  let resp = client.get(ECARD_DETAIL_URL)
    .query(&[
      ("timeRange", "day"),
      ("beginTime", &begin.date().format("%Y-%m-%d").to_string()),
      ("endTime", &now.date().format("%Y-%m-%d").to_string()),
      ("time", ""),
      ("pageSize", "10"),
      ("pageIndex", "0"),
      ("random_number", "801"),
    ])
    .header("x-id-token", jwt)
    .send().await?;
  let data: Resp<EcardLong> = resp.json().await?;

  if data.code == 0 {
    for item in data.data.data.costList {
      println!("{} {}\n  {:7} {}\n", item.operationType, item.operationTitle, item.payment, item.occurrenceTime);
    }
    Ok(())
  } else {
    bail!(data.message.unwrap())
  }
}

pub async fn ecard(
  _config: &Config,
  storage: &Storage,
  is_detail: bool,
) -> Result<()> {
  let client = Client::new();

  let jwt = if let Some(jwt) = &storage.login.cookie_jwt {
    jwt
  } else {
    bail!("Please sign in first!");
  };

  if is_detail {
    ecard_long(jwt, &client).await?;
  } else {
    ecard_short(jwt, &client).await?;
  }
  Ok(())
}
