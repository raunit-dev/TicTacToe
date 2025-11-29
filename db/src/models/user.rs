use crate::Store;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUserRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUserResponse {
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl Store {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<GetUserResponse> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, password, email) VALUES ($1,$2,$3) RETURNING id, username,password,email",
            request.username,
            request.password,
            request.email,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(GetUserResponse { user: user })
    }

    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, password, email FROM users WHERE username = $1",
            request.username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(GetUserResponse { user: user })
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<GetUserResponse> {
      let user = sqlx::query_as!(User, "SELECT id, username, password, email FROM users WHERE id = $1", Uuid::parse_str(&id)?)
          .fetch_one(&self.pool)
          .await?;

      Ok(GetUserResponse {
          user: user,
      })
  }
}

