use std::{ops::Deref, str::FromStr};

use super::{Log, Product, Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub id: CartId,
    pub user: Option<User>,
    pub products: Vec<(Product, u16)>,
    pub services: Vec<(Service, DateTime<Utc>)>,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CartId(Uuid);

impl CartId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for CartId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
