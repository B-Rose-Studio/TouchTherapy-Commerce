use std::{ops::Deref, str::FromStr};

use crate::models::Log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Role<'a> {
    pub id: RoleId,
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

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    Read,
    Create,
    Modify,
    Delete,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoleId(Uuid);

impl RoleId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for RoleId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
