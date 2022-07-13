use anyhow::{bail, Result};
use serde::Deserialize;
use chrono::{Local, Duration};
use reqwest::Client;
use crate::data::ECARD_DETAIL_URL;
use super::Resp;

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
    for item in data.data.unwrap().data.costList {
      println!("{} {}\n  {:7} {}\n", item.operationType, item.operationTitle, item.payment, item.occurrenceTime);
    }
    Ok(())
  } else {
    bail!("[{}] {}", data.code, data.message.unwrap())
  }
}
