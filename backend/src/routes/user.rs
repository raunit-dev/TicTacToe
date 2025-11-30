use crate::middleware::JwtClaims;
use actix_web::{Result, web};
use db::Store;
use db::models::user::{CreateUserRequest, GetUserRequest, GetUserResponse};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        Self {
            sub,
            exp: 100000000000000000,
        }
    }
}

// create_user is equivalent to sign_up
pub async fn create_user(
    data: web::Data<Store>,
    request: web::Json<CreateUserRequest>,
) -> Result<web::Json<GetUserResponse>, actix_web::error::Error> {
    let store = data.into_inner();
    let user = store
        .create_user(request.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(web::Json(user))
}

pub async fn sign_in(
    data: web::Data<Store>, //query the database and get something from there
    request: web::Json<CreateUserRequest>, //web::Json<> its an middleware
) -> Result<web::Json<SignInResponse>, actix_web::error::Error> {
    let store = data.into_inner();
    let user = store
        .get_user(GetUserRequest {
            username: request.into_inner().username,
        })
        .await
        .map_err(|e| actix_web::error::ErrorForbidden(e.to_string()))?;
    let token = encode(
        &Header::default(),
        &Claims::new(user.user.id),
        &EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes()),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(web::Json(SignInResponse { token }))
}

pub async fn get_user(
    data: web::Data<Store>,
    claims: JwtClaims,
) -> Result<web::Json<MeResponse>, actix_web::error::Error> {
    let store = data.into_inner();
    let user = store
        .get_user_by_id(claims.0.sub)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(web::Json(MeResponse {
        username: user.user.username,
    }))
}
