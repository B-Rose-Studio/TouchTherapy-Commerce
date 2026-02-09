use std::str::FromStr;

use super::Category;
use super::{Log, Supplier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Product<'a> {
    pub id: ProductId,

    pub name: &'a str,
    pub description: &'a str,
    pub price: f64,
    pub quantity: u16,
    pub sku: &'a str,
    pub images: Vec<&'a str>,

    pub categories: Vec<Category<'a>>,
    pub supplier: Supplier<'a>,

    pub active: bool,
    pub log: Log,
}
