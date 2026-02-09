use crate::models::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Role<'a> {
    pub id: Uuid,
    pub name: &'a str,

    pub users_permissions: Vec<Permissions>,
    pub products_permissions: Vec<Permissions>,
    pub categories_permissions: Vec<Permissions>,
    pub services_permissions: Vec<Permissions>,
    pub appointments_permissions: Vec<Permissions>,
    pub equipments_permissions: Vec<Permissions>,
    pub suppliers_permissions: Vec<Permissions>,
    pub orders_permissions: Vec<Permissions>,
    pub carts_permissions: Vec<Permissions>,
    pub roles_permissions: Vec<Permissions>,

    pub log: Log,
}

#[derive(Serialize, Deserialize)]
pub enum Permissions {
    Read,
    Create,
    Modify,
    Delete,
}
