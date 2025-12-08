use super::{Category, Equipment, Log, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Service<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub duration: u8,
    pub price: f64,
    pub equipments: Vec<Equipment<'a>>,
    pub professionals: Vec<User<'a>>,
    pub categories: Vec<Category<'a>>,
    pub active: bool,
    pub log: Log,
}
