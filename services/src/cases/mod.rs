use thiserror::Error;

pub mod user;

#[async_trait::async_trait]
pub trait Service<DTO> {
    async fn run(&self, dto: &DTO) -> Result<(), ServiceError>;
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("fields invalids")]
    ValidateError,
}
