use super::{Log, PaymentMethod, PaymentStatus, Product, User};
use crate::models::Address;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Order<'a> {
    pub id: OrderId,

    pub client: User<'a>,
    pub products: Vec<ProductInfo<'a>>,
    pub address: Address<'a>,

    pub status: OrderStatus,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub total: f64,
    pub nfe: &'a str,

    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ProductInfo<'a> {
    #[serde(borrow)]
    pub product: Product<'a>,
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
