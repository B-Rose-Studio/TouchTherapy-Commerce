mod surrealdb;
use crate::Service;
pub use surrealdb::*;

pub enum MigrationActions {
    Run(String),
    RunAll,
    Reverte(String),
    RevertAll,
    Clean,
}

pub trait MigrationService: Service<Args = MigrationActions> {}
