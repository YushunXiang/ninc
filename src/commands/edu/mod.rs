pub mod eval;

use anyhow::Result;
use reqwest::Client;
use crate::{
  data::{Config, Storage},
  tools::auth::edu::auth
};

pub async fn edu(
  _config: &Config,
  storage: &Storage,
) -> Result<()> {
  let token = auth(storage).await?;

  let client = Client::new();

  let re = regex::Regex::new("e\">(.+)同学")?;
  let resp = client.get("https://jwxt.nwpu.edu.cn/student/home")
    .header("Cookie", format!("SESSION={}; __pstsid__={}", token.session, token.pstsid))
    .send().await?
    .text().await?;

  let name = re.captures(resp.as_str()).unwrap().get(1).unwrap().as_str();
  println!("欢迎登录教务系统，{}同学！", name);

  Ok(())
}
