use crate::models::Log;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub id: Uuid,

    pub name: String,
    pub phone: String,

    pub postal_code: String,
    pub street_address: String,
    pub number: String,
    pub district: String,
    pub city: String,
    pub state: String,
    pub complement: String,
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
