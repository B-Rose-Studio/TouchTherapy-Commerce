use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserAuth<'a> {
    pub id: Uuid,
    pub code: &'a str,
    pub code_type: CodeType,
    pub expiration_time: DateTime<Utc>,
    pub used: bool,
}

#[derive(Serialize, Deserialize)]
pub enum CodeType {
    Login,
    ConfirmationEmail,
}
