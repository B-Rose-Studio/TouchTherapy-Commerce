use std::str::FromStr;

use super::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SupplierId(Uuid);

impl SupplierId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Supplier<'a> {
    pub id: SupplierId,
    pub name: &'a str,
    pub cnpj: &'a str,
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub log: Log,
}
