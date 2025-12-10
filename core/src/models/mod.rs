use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod appointment;
mod auth;
mod cart;
mod category;
mod equipment;
mod order;
mod payment;
mod product;
mod roles;
mod service;
mod suppliers;
mod user;

pub use appointment::*;
pub use auth::*;
pub use cart::*;
pub use category::*;
pub use equipment::*;
pub use order::*;
pub use payment::*;
pub use product::*;
pub use roles::*;
pub use service::*;
pub use suppliers::*;
pub use user::*;

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
