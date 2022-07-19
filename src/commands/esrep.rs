use std::io::Write;
use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::Value;
use regex::Regex;
use reqwest::Client;
use crate::{
  data::{
    Storage,
    Config,
    ESREP_URL,
    ESREP_POST_URL,
    CONFIG_FILE,
    EsrepConfig::{School, Other, self},
    args::esrep::EsrepArg,
    ReportAtSchool,
    ReportOther,
  },
  tools::auth::esrep
};

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Params {
  sign: String,
  timeStamp: String,
}

struct Res {
  already: bool,
  params: Params,
}

async fn fetch_params(jsessionid: &str, client: &Client) -> Result<Res> {
  let re = Regex::new(r"url:'ry_util\.jsp\?sign=(.+)&timeStamp=(\d+)',")?;

  let html = client.get(ESREP_URL)
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .send().await?
    .text().await?;

  let already = html.find("您已提交今日填报，重新提交将覆盖上一次的信息。").is_some();

  let res = re.captures(&html).unwrap();

  Ok(Res {
    already,
    params: Params {
      sign: res.get(1).unwrap().as_str().to_string(),
      timeStamp: res.get(2).unwrap().as_str().to_string(),
    }
  })
}

async fn report(
  content: &EsrepConfig,
  args: EsrepArg,
  jsessionid: &str,
) -> Result<()> {
  let client = Client::new();
  let res = fetch_params(jsessionid, &client).await?;

  if res.already && !args.yes {
    loop {
      print!("Already reported today. Are you sure to report again? (Y/n) ");
      std::io::stdout().flush()?;
      let mut input = String::new();
      std::io::stdin().read_line(&mut input)?;
      match input.trim() {
        "Y" | "y" => {
          break;
        },
        "N" | "n" => {
          bail!("Aborted.");
        },
        _ => { }
      }
    }
  }

  let raw = client.post(ESREP_POST_URL)
    .query(&res.params)
    .form(&content)
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .header("Referer", ESREP_URL)
    .send().await?
    .text().await?
    .replace("－", "-"); // 😅

  let res: Value = serde_json::from_str(&raw)?;

  match &res["state"] {
    Value::String(code) => {
      if code == "1" {
        Ok(())
      } else {
        bail!("{}", res)
      }
    },
    Value::Number(code) => {
      if code.is_i64() {
        if code.as_i64().unwrap() == -1 {
          bail!("{}", res["err-msg"].as_str().unwrap());
        } else {
          bail!("{}", res);
        }
      } else {
        bail!("{}", res);
      }
    },
    _ => {
      bail!("{}", res);
    }
  }
}

pub async fn esrep(
  config: &mut Config,
  storage: &Storage,
  args: EsrepArg,
) -> Result<()> {
  if storage.login.cookie_tgc.is_none() {
    bail!("Please sign in first!");
  }
  if args.school && (args.code.is_some() || args.address.is_some() || args.college.is_some() || args.phone.is_some()) {
    bail!("--code, --address, --college and --phone are exclusive with --school!");
  }
  if config.esrep.is_none() {
    config.esrep = Some(if args.school {
      School(ReportAtSchool::new(storage))
    } else {
      Other(ReportOther::new(storage, args.code, args.address, args.college, args.phone))
    });
    config.save(CONFIG_FILE).await?;
    bail!("Please set the report content in config file first!");
  }
  let esrep = config.esrep.as_ref().unwrap();
  let jsessionid = esrep::auth(storage).await?;
  report(&esrep, args, &jsessionid).await?;
  Ok(())
}
