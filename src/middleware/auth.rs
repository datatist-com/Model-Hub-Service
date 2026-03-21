use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

use crate::errors::AppError;
use crate::models::token as token_model;

/// Authenticated session context extracted from the token DB.
#[derive(Debug, Clone)]
pub struct Claims {
    pub sub: String,       // user id
    #[allow(dead_code)]
    pub username: String,
    pub role: String,
    pub token: String,     // raw token string (for revocation / refresh)
}

/// Extract the raw token string from the request.
/// Priority: Authorization header (Bearer or raw) → X-Token header → ?token= query param.
fn extract_token_str(req: &HttpRequest) -> Result<String, AppError> {
    // 1. Authorization header
    if let Some(auth) = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
    {
        let t = auth.strip_prefix("Bearer ").unwrap_or(auth).to_string();
        if !t.is_empty() {
            return Ok(t);
        }
    }

    // 2. X-Token header
    if let Some(xt) = req
        .headers()
        .get("X-Token")
        .and_then(|v| v.to_str().ok())
    {
        let t = xt.trim().to_string();
        if !t.is_empty() {
            return Ok(t);
        }
    }

    // 3. ?token= query parameter
    if let Some(t) = req.query_string().split('&').find_map(|pair| {
        let mut kv = pair.splitn(2, '=');
        let key = kv.next()?;
        let val = kv.next()?;
        if key == "token" && !val.is_empty() {
            Some(val.to_string())
        } else {
            None
        }
    }) {
        return Ok(t);
    }

    Err(AppError::Unauthorized(
        "Missing token: provide Authorization header, X-Token header, or ?token= query param"
            .into(),
    ))
}

/// Extractor: validates token against the DB and returns Claims.
impl FromRequest for Claims {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<sqlx::SqlitePool>>().cloned();
        let token_result = extract_token_str(req);

        Box::pin(async move {
            let pool =
                pool.ok_or_else(|| AppError::Internal("Missing DB pool".into()))?;
            let token_str = token_result?;

            let info = token_model::find_active(&pool, &token_str)
                .await?
                .ok_or_else(|| AppError::Unauthorized("Token is invalid or expired".into()))?;

            if info.user_status != "active" {
                return Err(AppError::Forbidden("Account is frozen".into()));
            }

            Ok(Claims {
                sub: info.user_id,
                username: info.username,
                role: info.role,
                token: token_str,
            })
        })
    }
}

/// Require platform_admin role.
pub struct AdminOnly(pub Claims);

impl FromRequest for AdminOnly {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Claims::from_request(req, payload);
        Box::pin(async move {
            match fut.await {
                Ok(claims) if claims.role == "platform_admin" => Ok(AdminOnly(claims)),
                Ok(_) => Err(AppError::Forbidden("Admin access required".into())),
                Err(e) => Err(e),
            }
        })
    }
}

