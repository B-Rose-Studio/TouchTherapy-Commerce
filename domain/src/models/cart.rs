use super::{Log, Product, Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Cart<'a> {
    pub id: Uuid,
    #[serde(borrow)]
    pub user: Option<User<'a>>,
    pub products: Vec<(Product<'a>, u16)>,
    pub services: Vec<(Service<'a>, DateTime<Utc>)>,
    pub log: Log,
}
