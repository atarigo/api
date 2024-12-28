use crate::user::model::CreateUserDto;
use crate::user::model::UpdateUserDto;
use crate::user::model::User;
use crate::user::repository::{RepositoryError, UserRepository};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,

    #[error("User Conflict: {0}")]
    ConflictError(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] RepositoryError),

    #[error("Validation error: {0}")]
    ValidationError(&'static str),
    /*
    #[error("Permission denied")]
    PermissionDenied,
    */

    /*
    #[error("Cache error: {0}")]
    CacheError(&'static str),
    */
}

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    fn map_errors(err: RepositoryError) -> UserServiceError {
        match err {
            RepositoryError::UniqueViolation(_) => {
                UserServiceError::ConflictError("Username already exists")
            }
            RepositoryError::InvalidField(msg) => UserServiceError::ValidationError(msg),
            RepositoryError::NotFound => UserServiceError::NotFound,
            _ => UserServiceError::DatabaseError(err),
        }
    }

    pub async fn create_user(&self, user: CreateUserDto) -> Result<User, UserServiceError> {
        if user.username.trim().is_empty() {
            return Err(UserServiceError::ValidationError(
                "Username cannot be empty",
            ));
        }
        if user.email.trim().is_empty() {
            return Err(UserServiceError::ValidationError("Email cannot be empty"));
        }
        if !user.email.contains("@") {
            return Err(UserServiceError::ValidationError("Invalid email format"));
        }

        self.repository.create(user).await.map_err(Self::map_errors)
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        self.repository.list().await.map_err(Self::map_errors)
    }

    pub async fn update_user(
        &self,
        id: &str,
        user: UpdateUserDto,
    ) -> Result<User, UserServiceError> {
        if user.username.is_some() && user.email.is_some() {
            return Err(UserServiceError::ValidationError(
                "No fields could be change",
            ));
        }

        self.repository
            .update(id, user)
            .await
            .map_err(Self::map_errors)
    }

    pub async fn delete_user(&self, id: &str) -> Result<bool, UserServiceError> {
        self.repository.delete(id).await.map_err(Self::map_errors)
    }
}
