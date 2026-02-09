use super::{Service, ServiceError};
use chrono::Local;
use domain::models;
use dtos::CreateClientDTO;
use repository::user::UserRepository;
use uuid::Uuid;
use validator::Validate;

pub struct CreateClientService<R: UserRepository + Send + Sync> {
    repository: R,
}

#[async_trait::async_trait]
impl<R: UserRepository + Send + Sync> Service<CreateClientDTO> for CreateClientService<R> {
    async fn run(&self, dto: &CreateClientDTO) -> Result<(), ServiceError> {
        match dto.validate() {
            Err(_) => return Err(ServiceError::ValidateError),
            _ => {}
        }

        let new_user = models::User {
            id: Uuid::new_v4(),
            active: true,
            full_name: &format!("{} {}", dto.first_name, dto.last_name),
            email: &dto.email,
            password: &dto.password,
            phone: &dto.phone,
            cpf: &dto.cpf,
            birth: dto.birth.clone(),
            auth: models::UserAuth {
                id: (),
                code: (),
                code_type: (),
                expiration_time: (),
                used: (),
            },
            log: models::Log {
                created_at: Local::now().to_utc(),
                updated_at: Local::now().to_utc(),
            },
        };

        self.repository.create_user(user).await;

        Ok(())
    }
}
