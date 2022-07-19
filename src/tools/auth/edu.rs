use anyhow::{bail, Result};
use regex::Regex;
use reqwest::{ClientBuilder, redirect::Policy};
use crate::data::{Storage, AUTH_URL, USER_AGENT, EDU_LOGIN_URL};

pub struct EduToken {
  pub session: String,
  pub pstsid: String,
}

pub async fn auth(storage: &Storage) -> Result<EduToken> {
  let tgc = if let Some(tgc) = &storage.login.cookie_tgc {
    tgc
  } else {
    bail!("Please sign in first!");
  };

  let client = ClientBuilder::new().redirect(Policy::none()).build()?;

  // 1st request: get __pstsid__
  let resp = client.get(EDU_LOGIN_URL).send().await?;

  let pstsid_raw = resp.headers().get("set-cookie").unwrap().to_str()?;
  let pstsid = &pstsid_raw.split(';').next().unwrap()[11..];

  // 2nd request: get ticket (for auth)
  let resp = client.get(AUTH_URL)
    .query(&[("service", EDU_LOGIN_URL)])
    .header("user-agent", USER_AGENT)
    .header("cookie", format!("TGC={}", tgc))
    .send().await?;

  let re = Regex::new("ticket=(.+)")?;
  let ticket_raw = if let Some(raw) = resp.headers().get("location") {
    raw.to_str()?
  } else {
    bail!("Token is invalid, please sign in again!");
  };
  let ticket = re.captures(ticket_raw).unwrap().get(1).unwrap().as_str();

  // 3rd request: get SESSION with ticket & __pstsid__
  let resp = client.get(EDU_LOGIN_URL)
    .query(&[("ticket", ticket)])
    .header("Cookie", format!("__pstsid__={}", pstsid))
    .send().await?;

  let session_raw = resp.headers().get("set-cookie").unwrap().to_str()?;
  let session = &session_raw.split(';').next().unwrap()[8..];

  // 4th request: sign in
  client.get(EDU_LOGIN_URL)
    .header("Cookie", format!("SESSION={}; __pstsid__={}", session, pstsid))
    .send().await?;

  Ok(EduToken {
    session: session.to_string(),
    pstsid: pstsid.to_string(),
  })
}
