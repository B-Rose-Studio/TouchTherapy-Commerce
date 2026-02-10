//pub mod user;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait::async_trait]
pub trait Repository {
    type Entity;
    type Id;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity>;
    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>>;
    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity>;
    async fn update(&self, new_entity: Self::Entity) -> RepositoryResult<Self::Entity>;
    async fn delete(&self, id: Self::Id) -> RepositoryResult<()>;
}

#[derive(Debug)]
pub enum RepositoryError {
    EntityNotFound(String),
    DatabaseConnection,
    DataError,
}

impl domain::error::ErrorTrait for RepositoryError {
    type Out = ();

    fn error<T: Sized + serde::Serialize>(&self) -> domain::error::Error<Self::Out> {
        let mut error =
            domain::error::Error::new("DB_ERROR_UNKNOW", "Unknown error repository", ());

        match self {
            RepositoryError::DataError => {
                error.code = "DB_ERROR_DATA".into();
                error.description = "Invalid query send to database".into();
            }

            RepositoryError::DatabaseConnection => {
                error.code = "DB_ERROR_CONNECTION".into();
                error.description = "Database not responde".into();
            }

            RepositoryError::EntityNotFound(e) => {
                error.code = "DB_ERROR_NOTFOUDN".into();
                error.description = format!("The entity '{0}' not found", e)
            }
        }

        error
    }
}
