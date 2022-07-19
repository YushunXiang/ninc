use serde::{Serialize, Deserialize};
use crate::data::Storage;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ReportOther {
  #[doc = "近 48 小时内是否进行过核酸检测。"]
  #[doc = "0: 否，1: 是。"]
  #[doc = "目前无需填写，缺省为 1。"]
  hsjc: Option<i32>,

  #[doc = "不知道什么玩意儿，缺省为 addRbxx。"]
  actionType: Option<String>,

  #[doc = "学号。"]
  userLoginId: Option<String>,

  #[doc = "当前所在位置。"]
  #[doc = "1: 在学校，2: 在西安，3: 在国内，4: 在国外。"]
  szcsbm: Option<i32>,

  #[doc = "不知道什么玩意儿，缺省为 1。"]
  bdzt: Option<i32>,

  #[doc = "当前所在位置具体地址。"]
  szcsmc: Option<String>,

  #[doc = "今天的体温范围。"]
  #[doc = "0: 37.3 度以下，1: 37.3 度 ~ 37.8 度，2: 37.8 ~ 39.0 度，3: 39.0 度以上。"]
  sfyzz: Option<i32>,

  #[doc = "疑似症状。"]
  #[doc = "用半角逗号分隔的列表。"]
  #[doc = "0: 无，1: 发热，6: 胸闷，7: 咳嗽，8: 其他症状（咽痛、呼吸困难、乏力、恶心呕吐、腹泻、结膜炎、肌肉酸痛等症状）。"]
  sfqz: Option<String>,

  #[doc = "不知道什么玩意儿，看名字像是填报来源(?)，缺省为 sso。"]
  tbly: Option<String>,

  #[doc = "其它情况说明。"]
  qtqksm: Option<String>,

  #[doc = "疑似症状情况说明。"]
  ycqksm: Option<String>,

  #[doc = "不知道什么玩意儿，看名字像是用户类型(?)，缺省为 2。"]
  userType: Option<i32>,

  #[doc = "姓名。"]
  userName: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为空。"]
  sfczbcqca: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为空。"]
  czbcqcasjd: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为空。"]
  sfczbcfhyy: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为空。"]
  czbcfhyysjd: Option<String>,

  #[doc = "当前所在位置具体地址。"]
  #[doc = "我特么也不知道为什么有两个…"]
  szcsmc1: Option<String>,

  #[doc = "近 15 天是否前往或经停中高风险地区，或其它有新冠患者病例报告的社区。"]
  #[doc = "0: 否，1: 中高风险地区，3: 其它有病例报告的社区。"]
  sfjt: Option<i32>,

  #[doc = "简单描述前往或经停的具体情况。"]
  sfjtsm: Option<String>,

  #[doc = "近 15 天是否接触过出入或居住在中高风险地区的人员，以及其他有新冠患者病例报告的发热或呼吸道症状患者。"]
  #[doc = "0: 否，1: 中高风险地区，3: 其他有病例社区的发热或呼吸道症状患者。"]
  sfjcry: Option<i32>,

  #[doc = "简单描述此类接触的具体情况。"]
  sfjcrysm: Option<String>,

  #[doc = "近 15 天您或家属是否接触过疑似或确诊患者，或无症状感染患者（核酸检测阳性者）。"]
  #[doc = "0: 否，1: 是。"]
  sfjcqz: Option<i32>,

  #[doc = "简单描述此类接触的具体情况。"]
  sfjcqzsm: Option<String>,

  #[doc = "您或家属是否正根据上级单位、医院相关要求进行居家或封闭性隔离。"]
  #[doc = "0: 正常学习生活，1: 居家（宿舍）隔离，2: 在校医院隔离，3: 定点医院隔离，9: 指定酒店/宿舍/特定区域隔离。"]
  glqk: Option<i32>,

  #[doc = "隔离开始日期。"]
  glksrq: Option<String>,
  
  #[doc = "隔离结束日期。"]
  gljsrq: Option<String>,
  
  #[doc = "隔离原因。"]
  glyy: Option<String>,

  #[doc = "您或家属当前健康状态。"]
  #[doc = "0: 正常，2: 疑似，3: 确诊，4: 已治愈，5: 疑似转排除（留观）。"]
  sfjkqk: Option<i32>,

  #[doc = "简单描述一下您或家属的健康情况。"]
  jkqksm: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为空。"]
  sfmtbg: Option<String>,

  #[doc = "不知道什么玩意儿，缺省为 3。"]
  qrlxzt: Option<i32>,

  #[doc = "学院名称，如“信息类”。"]
  xymc: Option<String>,

  #[doc = "学生手机号码。"]
  xssjhm: Option<String>,
}

impl ReportOther {
  pub fn new(
    storage: &Storage,
    code: Option<i32>,
    address: Option<String>,
    college: Option<String>,
    phone: Option<String>,
  ) -> Self {
    let uid = if let Some(uid) = storage.basic.uid.as_ref() {
      uid.clone()
    } else {
      String::from("1145141919")
    };

    let name = if let Some(name) = storage.basic.name.as_ref() {
      name.clone()
    } else {
      String::from("田所")
    };

    let code = if let Some(code) = code {
      code
    } else {
      610116
    };

    let address = if let Some(address) = address {
      address
    } else {
      String::from("下北泽")
    };

    let college = if let Some(college) = college {
      college
    } else {
      String::from("信息类")
    };

    let phone = if let Some(phone) = phone {
      phone
    } else {
      String::from("13888888888")
    };

    Self {
      hsjc: Some(1),
      sfczbcqca: Some(String::new()),
      czbcqcasjd: Some(String::new()),
      sfczbcfhyy: Some(String::new()),
      czbcfhyysjd: Some(String::new()),
      actionType: Some(String::from("addRbxx")),
      userLoginId: Some(uid),
      userName: Some(name),
      szcsbm: Some(code),
      szcsmc: Some(address.clone()),
      szcsmc1: Some(address),
      sfjt: Some(0),
      sfjtsm: Some(String::new()),
      sfjcry: Some(0),
      sfjcrysm: Some(String::new()),
      sfjcqz: Some(0),
      sfyzz: Some(0),
      sfqz: Some(String::from("0")),
      ycqksm: Some(String::new()),
      glqk: Some(0),
      glksrq: Some(String::new()),
      gljsrq: Some(String::new()),
      tbly: Some(String::from("sso")),
      glyy: Some(String::new()),
      qtqksm: Some(String::from("无")),
      sfjcqzsm: Some(String::new()),
      sfjkqk: Some(0),
      jkqksm: Some(String::new()),
      sfmtbg: Some(String::new()),
      userType: Some(2),
      qrlxzt: Some(3),
      bdzt: Some(1),
      xymc: Some(college),
      xssjhm: Some(phone),
    }
  }
}
