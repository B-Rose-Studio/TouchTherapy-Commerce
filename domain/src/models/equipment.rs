use super::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Equipment<'a> {
    pub id: EquipmentId,
    pub name: &'a str,
    pub serial: &'a str,
    pub status: EquipmentStatus,
    pub quantity: u16,
    pub purchased_at: DateTime<Utc>,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum EquipmentStatus {
    Operational,
    Broken,
    Missing,
    Maintenance,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EquipmentId(Uuid);

impl EquipmentId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for EquipmentId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
