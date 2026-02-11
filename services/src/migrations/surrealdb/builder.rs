use crate::{ServiceBuilder, migrations::SurrealDbMigrationService};
use std::sync::Arc;
use surrealdb::{Surreal, engine::any::Any};

#[derive(Default)]
pub struct SurrealDbMigrationBuilder {
    pub db: Option<Arc<Surreal<Any>>>,
    pub path: Option<String>,
}

impl SurrealDbMigrationBuilder {
    pub fn db(mut self, db: Arc<Surreal<Any>>) -> Self {
        self.db = Some(db);
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
}

impl ServiceBuilder for SurrealDbMigrationBuilder {
    type S = SurrealDbMigrationService;

    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Self::S {
        Self::S {
            db: self.db.expect(""),
            path: self.path.expect(""),
        }
    }
}
