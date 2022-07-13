#![allow(non_snake_case, dead_code)]
mod short;
mod long;

use anyhow::{bail, Result};
use serde::Deserialize;
use chrono::{Local, Duration, NaiveDate};
use reqwest::Client;
use crate::data::{Config, Storage, args::EcardArg};
use self::{short::ecard_short, long::ecard_long};

#[derive(Deserialize)]
struct Resp<T> {
  code: i32,
  data: Option<T>,
  message: Option<String>,
}

pub async fn ecard(
  _config: &Config,
  storage: &Storage,
  args: EcardArg,
) -> Result<()> {
  let client = Client::new();

  let jwt = if let Some(jwt) = &storage.login.cookie_jwt {
    jwt
  } else {
    bail!("Please sign in first!");
  };

  if args.detail {
    let end = if let Some(end_raw) = args.end {
      if let Ok(res) = NaiveDate::parse_from_str(&end_raw, "%Y-%m-%d") {
        res
      } else {
        bail!("--end invalid! Please use format like `2003-04-17`.");
      }
    } else {
      Local::now().naive_local().date()
    };

    let begin = if let Some(begin_raw) = args.begin {
      if let Ok(res) = NaiveDate::parse_from_str(&begin_raw, "%Y-%m-%d") {
        res
      } else {
        bail!("--begin invalid! Please use format like `2003-04-17`.");
      }
    } else {
      end - Duration::days(30)
    };

    let num = args.limit.unwrap_or(10);

    ecard_long(jwt, &client, begin, end, num).await?;
  } else {
    if args.begin.is_some() || args.end.is_some() || args.limit.is_some() {
      bail!("--begin, --end and --limit are only supported with --detail!");
    }
    ecard_short(jwt, &client).await?;
  }
  Ok(())
}
