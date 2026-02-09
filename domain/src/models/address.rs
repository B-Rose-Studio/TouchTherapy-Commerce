use crate::models::Log;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Address<'a> {
    pub id: Uuid,

    pub name: &'a str,
    pub phone: &'a str,

    pub postal_code: &'a str,
    pub street_address: &'a str,
    pub number: &'a str,
    pub district: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub complement: &'a str,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddressId(Uuid);

impl AddressId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for AddressId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
