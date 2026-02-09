use super::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Supplier<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub cnpj: &'a str,
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub log: Log,
}
