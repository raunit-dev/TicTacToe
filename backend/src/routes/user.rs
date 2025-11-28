use actix_web::{HttpRequest, Result, web};
use db::Store;
use db::models::user::{CreateUserRequest, GetUserRequest, GetUserResponse};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
            exp: 1000000000,
        }
    }
}

// create_user is equivalent to sign_in
pub async fn create_user(
    data: web::Data<Store>,
    request: web::Json<CreateUserRequest>,
) -> Result<web::Json<GetUserResponse>> {
    let store = data.into_inner();
    let user = store
        .create_user(request.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(web::Json(user))
}

pub async fn sign_in(
    data: web::Data<Store>,
    request: web::Json<CreateUserRequest>,
) -> Result<web::Json<SignInResponse>> {
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

pub async fn me(data: web::Data<Store>, request: HttpRequest) -> Result<web::Json<MeResponse>> {
    let store = data.into_inner();
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?
        .to_str()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Header Encoding"))?;
    let token = auth_header
        .strip_prefix("Bearer")
        .unwrap_or(auth_header)
        .trim();
    if token.is_empty() {
        return Err(actix_web::error::ErrorUnauthorized("Empty Token"));
    }

    let secret = env::var("SECRET_KEY")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Missing Secret Key"))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or Expired Token"))?;

    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid User id in token"))?;

    let user = store
        .get_user_by_id(user_id)
        .await
        .map_err(|e| actix_web::error::ErrorForbidden(e.to_string()))?;
    let username = user.user.username;

    Ok(web::Json(MeResponse { username }))
}
