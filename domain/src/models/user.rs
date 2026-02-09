use super::{Log, Role};
use crate::models::Address;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct User<'a> {
    pub id: UserId,

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for UserId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(uuid) = Uuid::from_str(s) {
            return Ok(Self(uuid));
        }

        Err(())
    }
}
