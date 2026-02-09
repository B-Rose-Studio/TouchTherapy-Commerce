use crate::models::Address;

use super::{Log, PaymentMethod, PaymentStatus, Product, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Order<'a> {
    pub id: Uuid,

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

#[derive(Serialize, Deserialize)]
pub struct ProductInfo<'a> {
    #[serde(borrow)]
    pub product: Product<'a>,
    pub quantity: u16,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub enum OrderStatus {
    Preparing,
    Sent,
    Delivered,
    Reversed,
}
