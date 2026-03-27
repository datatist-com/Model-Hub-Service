use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::Value;
use std::fmt;

// ── i18n message carrier ──

#[derive(Debug, Clone)]
pub struct I18nMsg {
    pub key: String,
    pub params: Option<Value>,
}

impl I18nMsg {
    pub fn with_params(key: impl Into<String>, params: Value) -> Self {
        Self { key: key.into(), params: Some(params) }
    }
}

impl From<&str> for I18nMsg {
    fn from(s: &str) -> Self {
        Self { key: s.to_string(), params: None }
    }
}

impl From<String> for I18nMsg {
    fn from(s: String) -> Self {
        Self { key: s, params: None }
    }
}

// ── Application error ──

#[derive(Debug)]
pub enum AppError {
    BadRequest(I18nMsg),
    Unauthorized(I18nMsg),
    Forbidden(I18nMsg),
    NotFound(I18nMsg),
    Conflict(I18nMsg),
    Internal(I18nMsg),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            Self::BadRequest(m) | Self::Unauthorized(m) | Self::Forbidden(m)
            | Self::NotFound(m) | Self::Conflict(m) | Self::Internal(m) => &m.key,
        };
        f.write_str(key)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code, msg) = match self {
            Self::BadRequest(m) => (actix_web::http::StatusCode::BAD_REQUEST, "BAD_REQUEST", m),
            Self::Unauthorized(m) => {
                (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED", m)
            }
            Self::Forbidden(m) => (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN", m),
            Self::NotFound(m) => (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND", m),
            Self::Conflict(m) => (actix_web::http::StatusCode::CONFLICT, "CONFLICT", m),
            Self::Internal(m) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                m,
            ),
        };
        HttpResponse::build(status).json(ApiResponse::<()> {
            code: code.into(),
            message: msg.key.clone(),
            params: msg.params.clone(),
            data: None,
        })
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {e}");
        match e {
            sqlx::Error::RowNotFound => Self::NotFound("error.resource.not_found".into()),
            sqlx::Error::Database(ref db_err)
                if matches!(db_err.kind(), sqlx::error::ErrorKind::UniqueViolation) =>
            {
                Self::Conflict("error.resource.conflict".into())
            }
            _ => Self::Internal("error.internal".into()),
        }
    }
}

// ── Unified response envelope ──

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T, message: &str) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            code: "OK".into(),
            message: message.into(),
            params: None,
            data: Some(data),
        })
    }

    pub fn ok_params(data: T, message: &str, params: Value) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            code: "OK".into(),
            message: message.into(),
            params: Some(params),
            data: Some(data),
        })
    }
}

#[derive(Serialize)]
pub struct PagedData<T: Serialize> {
    pub items: Vec<T>,
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub total: i64,
}
