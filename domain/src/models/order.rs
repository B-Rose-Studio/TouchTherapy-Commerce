use super::{Log, PaymentMethod, PaymentStatus, Product, User};
use crate::models::Address;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: OrderId,

    pub client: User,
    pub products: Vec<ProductInfo>,
    pub address: Address,

    pub status: OrderStatus,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub total: f64,
    pub nfe: String,

    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ProductInfo {
    pub product: Product,
    pub quantity: u16,
    pub price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    Preparing,
    Sent,
    Delivered,
    Reversed,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for OrderId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
