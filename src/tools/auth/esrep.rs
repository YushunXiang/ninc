use anyhow::{bail, Result};
use regex::Regex;
use reqwest::{ClientBuilder, redirect::Policy};
use crate::data::{Storage, ESREP_LOGIN_URL, AUTH_URL, SERVICE_ESREP};

pub async fn auth(storage: &Storage) -> Result<String> {
  let tgc = if let Some(tgc) = &storage.login.cookie_tgc {
    tgc
  } else {
    bail!("Please sign in first!");
  };

  let client = ClientBuilder::new().redirect(Policy::none()).build()?;

  // first request: get JSESSIONID
  let resp = client.get(ESREP_LOGIN_URL).send().await?;

  let jsessionid_raw = resp.headers().get("set-cookie").unwrap().to_str()?;
  let jsessionid = &jsessionid_raw.split(';').next().unwrap()[11..];

  // second request: get ticket (auth)
  let resp = client.get(AUTH_URL)
    .query(&[("service", SERVICE_ESREP)])
    .header("cookie", format!("TGC={}", tgc))
    .send().await?;

  let re = Regex::new("ticket=(.+)")?;
  let ticket_raw = resp.headers().get("location").unwrap().to_str()?;
  let ticket = re.captures(ticket_raw).unwrap().get(1).unwrap().as_str();

  // third request: sign in with ticket & JSESSIONID
  client.get(ESREP_LOGIN_URL)
    .query(&[("ticket", ticket)])
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .send().await?;

  Ok(String::from(jsessionid))
}
