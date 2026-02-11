mod builder;
mod service;

pub use builder::*;
use domain::error::ErrorTrait;
pub use service::*;

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
