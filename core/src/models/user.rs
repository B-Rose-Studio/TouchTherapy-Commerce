use super::{Log, Role, UserAuth};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Uuid,

    pub password: &'a str,
    pub auth: UserAuth<'a>,

    pub full_name: &'a str,
    pub email: &'a str,
    pub cpf: &'a str,
    pub phone: &'a str,
    pub birth: DateTime<Utc>,

    pub role: Role<'a>,
    pub active: bool,

    pub log: Log,
}
