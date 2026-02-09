use std::{ops::Deref, str::FromStr};

use super::{Log, Product, Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Cart<'a> {
    pub id: CartId,
    #[serde(borrow)]
    pub user: Option<User<'a>>,
    pub products: Vec<(Product<'a>, u16)>,
    pub services: Vec<(Service<'a>, DateTime<Utc>)>,
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
