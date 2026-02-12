use std::{path::PathBuf, sync::Arc};
use surrealdb::{Surreal, engine::any::Any};
use tokio::{fs::File, io::AsyncReadExt};

const QUERY_CREATE_TABLE: &str = "
DEFINE TABLE OVERWRITE _migrations SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE script_name ON _migrations TYPE string;
DEFINE FIELD OVERWRITE executed_at ON _migrations TYPE datetime VALUE time::now() READONLY;
";

const QUERY_CREATE_MIGRATION: &str = "
INSERT INTO _migrations {
    script_name: $name
};
";

const QUERY_DELETE_MIGRATION: &str = "
DELETE _migrations WHERE script_name = $name;
";

const QUERY_CLEAN_MIGRATIONS: &str = "
DELETE _migrations;
";

use crate::{
    Service,
    migrations::{MigrationActions, MigrationError, MigrationRegister, MigrationService},
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

        let migrations: Vec<MigrationRegister> = self
            .db
            .select("_migrations")
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        match args {
            MigrationActions::Run(name) => {
                self.apply_migration(&name, &migrations).await?;
            }

            MigrationActions::RunAll => {}

            MigrationActions::Reverte(name) => {
                self.revert_migration(&name, &migrations).await?;
            }

            MigrationActions::RevertAll => {}

            MigrationActions::Clean => {
                self.db
                    .query(QUERY_CLEAN_MIGRATIONS)
                    .await
                    .map_err(|e| MigrationError(e.to_string()))?;
            }
        }

        Err(MigrationError("Not Implemented".into()))
    }
}

impl MigrationService for SurrealDbMigrationService {}

impl SurrealDbMigrationService {
    async fn apply_migration(
        &self,
        name: &str,
        migrations: &[MigrationRegister],
    ) -> Result<(), MigrationError> {
        if migrations.iter().any(|m| m.script_name == name) {
            return Ok(());
        }

        let mut path = PathBuf::from(&self.path);
        path.push(name);

        let mut file_script = File::open(&path)
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        let mut query = String::new();
        let _ = file_script.read_to_string(&mut query).await;

        self.db
            .query(query)
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        self.db
            .query(QUERY_CREATE_MIGRATION)
            .bind(("name", name.to_string()))
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        Ok(())
    }

    async fn revert_migration(
        &self,
        name: &str,
        migrations: &[MigrationRegister],
    ) -> Result<(), MigrationError> {
        if !migrations.iter().any(|m| m.script_name == name) {
            return Ok(());
        }

        let mut path = PathBuf::from(&self.path);
        path.push(format!("!{name}"));

        let mut file_script = File::open(&path)
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        let mut query = String::new();
        let _ = file_script.read_to_string(&mut query).await;

        self.db
            .query(QUERY_DELETE_MIGRATION)
            .bind(("name", name.to_string()))
            .await
            .map_err(|e| MigrationError(e.to_string()))?;

        Ok(())
    }
}
