mod report_at_school;
mod report_other;

use serde::{Serialize, Deserialize};
pub use report_at_school::*;
pub use report_other::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EsrepConfig {
  School(ReportAtSchool),
  Other(ReportOther),
}
