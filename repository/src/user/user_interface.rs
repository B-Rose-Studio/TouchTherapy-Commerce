use domain::models::User;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_user_by_id<'a>(&self, id: Uuid) -> Result<Option<User<'a>>, ()>;
    async fn list_users<'a>(&self, quantity: usize, page: usize) -> Result<Vec<User<'a>>, ()>;
    async fn create_user<'a>(&self, user: User<'a>) -> Result<User<'a>, ()>;
    async fn update_user<'a>(&self, new_user: User<'a>) -> Result<User<'a>, ()>;
    async fn delete_user(&self, id: Uuid) -> Result<(), ()>;
}
