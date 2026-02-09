use crate::models::Address;

use super::{Log, Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Uuid,

    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub cpf: &'a str,
    pub phone: &'a str,
    pub birth: DateTime<Utc>,

    pub addresses: Vec<Address<'a>>,

    pub verified: bool,
    pub otp: bool,
    pub secret_key: Option<&'a str>,

    pub role: Role<'a>,
    pub active: bool,

    pub log: Log,
}
