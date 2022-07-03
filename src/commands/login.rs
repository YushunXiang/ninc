use anyhow::{Result, bail};
use regex::Regex;

pub async fn login(
  username: &str,
  password: &str
) -> Result<()> {
  let client = reqwest::Client::new();

  // let pubkey = client.get("https://uis.nwpu.edu.cn/cas/jwt/publicKey").send().await?.text().await?;

  let re = Regex::new("<input type=\"hidden\" name=\"execution\" value=\"(.+?)\"/>").unwrap();
  let execution_raw = client.get("https://uis.nwpu.edu.cn/cas/login").send().await?.text().await?;
  let execution = re.captures(&execution_raw).unwrap().get(1).unwrap().as_str();

  let login_resp = client.post("https://uis.nwpu.edu.cn/cas/login")
    .form(&[
      ("username", username),
      ("password", password),
      ("execution", execution),
      ("_eventId", "submit"),
    ])
    .send().await?;

  if login_resp.status() == 401 {
    bail!("Sign in failed!\nPlease check your username and password.");
  }

  let cookie_raw = login_resp.headers().get("set-cookie").unwrap().to_str().unwrap();
  let cookie_tgc = &cookie_raw.split(';').next().unwrap()[4..];

  println!("Sign in successful!\nTGC: {}", cookie_tgc);
  // TODO: save cookie to file

  Ok(())
}
