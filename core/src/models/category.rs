use super::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Category<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub log: Log,
}
