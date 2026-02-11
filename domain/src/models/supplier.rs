use std::{ops::Deref, str::FromStr};

use super::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Supplier {
    pub id: SupplierId,
    pub name: String,
    pub cnpj: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SupplierId(Uuid);

impl SupplierId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for SupplierId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
