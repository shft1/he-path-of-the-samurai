use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

pub type ApiResult<T> = Result<Json<ApiEnvelope<T>>, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiEnvelope<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<ErrorBody>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub body: ErrorBody,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            body: ErrorBody {
                code: code.into(),
                message: message.into(),
                trace_id: Uuid::new_v4().to_string(),
            },
        }
    }

    pub fn from_error(code: &'static str, err: impl std::error::Error) -> Self {
        Self::new(code, err.to_string())
    }

    pub fn tracing_code(&self) -> &str {
        &self.body.trace_id
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.body.code, self.body.message)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let payload = ApiEnvelope::<serde_json::Value> {
            ok: false,
            data: None,
            error: Some(self.body),
        };
        (StatusCode::OK, Json(payload)).into_response()
    }
}

impl<T: Serialize> ApiEnvelope<T> {
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }
}

pub fn respond<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(Json(ApiEnvelope::ok(data)))
}

