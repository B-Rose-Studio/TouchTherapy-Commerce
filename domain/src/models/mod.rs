use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod address; // X
mod appointment; // X
mod cart; //X
mod category; // X
mod equipment; // X
mod order; // X
mod payment; // X
mod product; // X
mod roles; // X
mod service; // X
mod supplier; // X
mod user; // X

pub use address::*;
pub use appointment::*;
pub use cart::*;
pub use category::*;
pub use equipment::*;
pub use order::*;
pub use payment::*;
pub use product::*;
pub use roles::*;
pub use service::*;
pub use supplier::*;
pub use user::*;

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
