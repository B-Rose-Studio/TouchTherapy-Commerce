use std::{ops::Deref, str::FromStr};

use super::{Category, Equipment, Log, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Service<'a> {
    pub id: ServiceId,

    pub name: &'a str,
    pub description: &'a str,
    pub images: Vec<&'a str>,
    pub duration: u8,
    pub price: f64,
    pub sessions: u8,

    pub equipments: Vec<Equipment<'a>>,
    pub professionals: Vec<User<'a>>,
    pub categories: Vec<Category<'a>>,

    pub active: bool,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceId(Uuid);

impl ServiceId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for ServiceId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
