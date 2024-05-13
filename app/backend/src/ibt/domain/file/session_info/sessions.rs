pub mod session;

use crate::ibt::domain::file::session_info::sessions::session::Session;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sessions {
    pub sessions: Option<Vec<Session>>,
}