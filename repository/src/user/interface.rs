use crate::Repository;
use domain::models::{User, UserId};

#[async_trait::async_trait]
pub trait UserRepository: Repository<Entity = User, Id = UserId> {}
