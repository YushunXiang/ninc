use anyhow::{bail, Result};
use serde::Deserialize;
use regex::Regex;
use reqwest::{ClientBuilder, redirect::Policy};
use crate::data::{AUTH_URL, Storage, STORAGE_FILE};

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
struct Res {
  identityTypeCode: String,
  aud: String,
  sub: String,
  organizationCode: String,
  iss: String,
  idToken: String,
  exp: usize,
  iat: usize,
  jti: String,
}

pub async fn auth(storage: &mut Storage) -> Result<()> {
  let client = ClientBuilder::new().redirect(Policy::none()).build()?;

  if let Some(tgc) = &storage.login.cookie_tgc {
    let resp = client.get(AUTH_URL)
      .header("cookie", format!("TGC={}", tgc))
      .send().await?;
    if resp.status() == 302 {
      let re = Regex::new(r"ticket=(.+)").unwrap();

      let jwt_raw = resp.headers().get("location").unwrap().to_str().unwrap();
      let jwt = re.captures(jwt_raw).unwrap().get(1).unwrap().as_str();

      let payload_raw = jwt.split('.').nth(1).unwrap().replace("%3D", "=");
      let payload: Res = serde_json::from_slice(&base64::decode(payload_raw)?).unwrap();

      storage.login.cookie_jwt = Some(String::from(payload.idToken));
      storage.save(STORAGE_FILE).await?;

      Ok(())
    } else {
      bail!("HTTP Status code: {}", resp.status());
    }
  } else {
    bail!("Please sign in first!");
  }
}
