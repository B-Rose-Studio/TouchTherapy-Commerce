use std::sync::Arc;
use surrealdb::{Surreal, engine::any::Any};

const QUERY_CREATE_TABLE: &str = "
DEFINE TABLE OVERWRITE script_migration SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE script_name ON script_migration TYPE string;
DEFINE FIELD OVERWRITE executed_at ON script_migration TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE checksum ON script_migration TYPE option<string>;
";

use crate::{
    Service,
    migrations::{MigrationActions, MigrationError, MigrationService},
};

pub struct SurrealDbMigrationService {
    pub db: Arc<Surreal<Any>>,
    pub path: String,
}

impl SurrealDbMigrationService {
    pub fn new(db: Arc<Surreal<Any>>, path: String) -> Self {
        Self { db, path }
    }
}

#[async_trait::async_trait]
impl Service for SurrealDbMigrationService {
    type Args = MigrationActions;
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, impl domain::error::ErrorTrait> {
        self.db
            .query(QUERY_CREATE_TABLE)
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        match args {
            MigrationActions::Run(_name) => {}

            MigrationActions::RunAll => {}

            MigrationActions::Reverte(_name) => {}

            MigrationActions::RevertAll => {}

            MigrationActions::Clean => {}
        }

        Err(MigrationError("Not Implemented".into()))
    }
}

impl MigrationService for SurrealDbMigrationService {}
