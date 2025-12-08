use super::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Equipment<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub serial: &'a str,
    pub status: EquipmentStatus,
    pub quantity: u16,
    pub purchased_at: DateTime<Utc>,
    pub log: Log,
}

#[derive(Serialize, Deserialize)]
pub enum EquipmentStatus {
    Operational,
    Broken,
    Missing,
    Maintenance,
}
