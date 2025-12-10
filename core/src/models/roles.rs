use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Role<'a> {
    pub id: Uuid,
    pub name: &'a str,

    pub user_permissions: Vec<Permissions>,
    pub product_permissions: Vec<Permissions>,
    pub category_permissions: Vec<Permissions>,
    pub service_permissions: Vec<Permissions>,
    pub appointment_permissions: Vec<Permissions>,
    pub equipment_permissions: Vec<Permissions>,
    pub supplier_permissions: Vec<Permissions>,
    pub order_permissions: Vec<Permissions>,
}

#[derive(Serialize, Deserialize)]
pub enum Permissions {
    Read,
    Create,
    Modify,
    Delete,
}
