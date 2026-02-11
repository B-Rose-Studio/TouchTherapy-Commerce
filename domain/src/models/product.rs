use std::ops::Deref;
use std::str::FromStr;

use super::Category;
use super::{Log, Supplier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: ProductId,

    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u16,
    pub sku: String,
    pub images: Vec<String>,

    pub categories: Vec<Category>,
    pub supplier: Supplier,

    pub active: bool,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for ProductId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
