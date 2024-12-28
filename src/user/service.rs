use crate::user::model::CreateUserDto;
use crate::user::model::UpdateUserDto;
use crate::user::model::User;
use crate::user::repository::UserRepository;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, user: CreateUserDto) -> Result<User, UserError> {
        self.repository.create(user).await.map_err(UserError::from)
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserError> {
        self.repository.list().await.map_err(UserError::from)
    }

    pub async fn update_user(&self, id: &str, user: UpdateUserDto) -> Result<User, UserError> {
        self.repository
            .update(id, user)
            .await
            .map_err(UserError::from)
    }
}
