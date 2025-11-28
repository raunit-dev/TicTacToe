use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Store;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUserRequest {
    username: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUserResponse {
    user: User,
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
}

impl Store {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<CreateUserResponse> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, password) VALUES ($1,$2) RETURNING id, username,password",
            request.username,
            request.password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(GetUserResponse { user: user })
    }

    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, password FROM users WHERE username = $1",
            request.username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(GetUserResponse { user: user })
    }
}
