use actix_web::{dev::Payload, FromRequest, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,     // user id
    pub username: String,
    pub role: String,
    pub exp: usize,      // expiry (unix timestamp)
}

impl Claims {
    pub fn new(user_id: &str, username: &str, role: &str, secret: &str) -> Result<String, AppError> {
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Self {
            sub: user_id.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, AppError> {
        decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))
    }
}

/// Extractor: pulls Claims from (in priority order):
///   1. Authorization header — "Bearer <token>" or raw "<token>"
///   2. X-Token header
///   3. ?token=<token> query parameter
impl FromRequest for Claims {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = (|| {
            let config = req
                .app_data::<actix_web::web::Data<crate::config::AppConfig>>()
                .ok_or_else(|| AppError::Internal("Missing app config".into()))?;

            // 1. Authorization header
            let token_from_auth = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .map(|v| {
                    if let Some(t) = v.strip_prefix("Bearer ") {
                        t.to_string()
                    } else {
                        v.to_string()
                    }
                });

            // 2. X-Token header
            let token_from_x = req
                .headers()
                .get("X-Token")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string());

            // 3. ?token= query parameter
            let token_from_query = req
                .query_string()
                .split('&')
                .find_map(|pair| {
                    let mut kv = pair.splitn(2, '=');
                    let key = kv.next()?;
                    let val = kv.next()?;
                    if key == "token" && !val.is_empty() {
                        Some(val.to_string())
                    } else {
                        None
                    }
                });

            let token = token_from_auth
                .or(token_from_x)
                .or(token_from_query)
                .ok_or_else(|| AppError::Unauthorized(
                    "Missing token: provide Authorization header, X-Token header, or ?token= query param".into(),
                ))?;

            Claims::decode(&token, &config.jwt_secret)
        })();

        ready(result)
    }
}

/// Require platform_admin role.
pub struct AdminOnly(pub Claims);

impl FromRequest for AdminOnly {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let claims_result = Claims::from_request(req, payload).into_inner();
        ready(match claims_result {
            Ok(claims) if claims.role == "platform_admin" => Ok(AdminOnly(claims)),
            Ok(_) => Err(AppError::Forbidden("Admin access required".into())),
            Err(e) => Err(e),
        })
    }
}
