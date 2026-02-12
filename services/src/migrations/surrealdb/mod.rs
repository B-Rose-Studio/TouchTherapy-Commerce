mod builder;
mod service;

pub use builder::*;
pub use service::*;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct MigrationRegister {
    script_name: String,
    execited_at: chrono::DateTime<chrono::Utc>,
}

use domain::error::ErrorTrait;
pub struct MigrationError(pub String);

impl ErrorTrait for MigrationError {
    fn error<T: Sized + serde::Serialize>(&self) -> domain::error::Error {
        domain::error::Error::new(
            "SERVER_MIGRATION_ERROR",
            "error in running migrations",
            self.0.clone(),
        )
    }
}
