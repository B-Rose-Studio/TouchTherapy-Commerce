use std::str::FromStr;

use super::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CategoryId(Uuid);

impl CategoryId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Category<'a> {
    pub id: CategoryId,
    pub name: &'a str,
    pub log: Log,
}
