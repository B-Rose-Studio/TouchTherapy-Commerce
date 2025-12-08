use super::{Log, Supplier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Product<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub price: f64,
    pub quantity: u16,
    pub sku: &'a str,
    pub imagens: Vec<&'a str>,
    pub supplier: Supplier<'a>,
    pub active: bool,
    pub log: Log,
}
