use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod appointment;
mod category;
mod equipment;
mod product;
mod roles;
mod service;
mod suppliers;
mod user;

pub use appointment::*;
pub use category::*;
pub use equipment::*;
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
