use std::cmp::min;
use anyhow::{bail, Result};
use serde::Deserialize;
use chrono::NaiveDate;
use reqwest::Client;
use crate::data::ECARD_DETAIL_URL;
use super::Resp;

#[derive(Deserialize)]
struct EcardOperation {
  blance: f64, // What the fxxk?
  occurrenceTime: String,
  operationTitle: Option<String>, // 😅
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

async fn fetch_record(
  jwt: &str,
  client: &Client,
  begin: NaiveDate,
  end: NaiveDate,
  page: usize,
) -> Result<EcardDetail> {
  let resp = client.get(ECARD_DETAIL_URL)
    .query(&[
      ("pageSize", "10"),
      ("pageIndex", &page.to_string()),
      ("beginTime", &begin.to_string()),
      ("endTime", &end.to_string()),
    ])
    .header("x-id-token", jwt)
    .send().await?;

  let data: Resp<EcardLong> = resp.json().await?;

  if data.code == 0 {
    Ok(data.data.unwrap().data)
  } else {
    bail!("[{}] {}", data.code, data.message.unwrap())
  }
}

pub async fn ecard_long(
  jwt: &str,
  client: &Client,
  begin: NaiveDate,
  end: NaiveDate,
  num: usize,
) -> Result<()> {
  let data = fetch_record(jwt, client, begin, end, 0).await?;
  let mut list = data.costList;

  let total_num = min(num, data.totalSum as usize);
  let pages = (total_num - 1) / 10 + 1;

  for page in 1..pages {
    loop {
      let data = fetch_record(jwt, client, begin, end, page).await;
      if data.is_ok() {
        list.append(&mut data.unwrap().costList);
        break;
      } else {
        eprintln!("Failed at page {}, retrying...\n{}", page, data.err().unwrap());
      }
    }
    println!("Now page {}/{}...", page + 1, pages);
  }

  println!("From: {}", begin);
  println!("To:   {}\n", end);

  for item in list {
    let title = item.operationTitle.unwrap_or("-".to_string());
    println!("{} {}\n  {:11} at {}\n", item.operationType, title, item.payment + " CNY", item.occurrenceTime);
  }
  println!("{} record{} in total.", total_num, if total_num > 1 { "s" } else { "" });

  Ok(())
}
