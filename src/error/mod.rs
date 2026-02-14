use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid port value: {0}")]
    InvalidPort(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Inavlid UUID: {0}")]
    InvalidUuid(String),
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        match self {
            ConfigError::InvalidUuid(msg) => {
                let error_response = json!({
                    "error": {
                        "code": "INVALID_UUID",
                        "status": 400,
                        "type": "ValidationError",
                        "message": msg,
                        "details": {
                            "expected_format": "UUID v4 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)",
                            "example": "123e4567-e89b-12d3-a456-426614174000"
                        },
                        "timestamp": timestamp
                    }
                });
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            }
            ConfigError::MissingEnv(var) => {
                let error_response = json!({
                    "error": {
                        "code": "ENV_MISSING",
                        "status": 500,
                        "type": "ConfigurationError",
                        "message": format!("Missing environment variable: '{}'", var),
                        "details": {
                            "variable": var,
                            "required": true,
                            "suggestion": "Check your .env file or environment variables"
                        },
                        "timestamp": timestamp
                    }
                });

                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }

            ConfigError::InvalidPort(port_value) => {
                let error_response = json!({
                    "error": {
                        "code": "INVALID_PORT",
                        "status": 400,
                        "type": "ValidationError",
                        "message": format!("Invalid port value: '{}'", port_value),
                        "details": {
                            "provided_value": port_value,
                            "expected_format": "number between 1-65535",
                            "examples": ["3000", "8080", "5432"],
                            "suggestion": "Provide a valid port number between 1 and 65535"
                        },
                        "timestamp": timestamp
                    }
                });

                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            }

            ConfigError::InternalServerError(msg) => {
                let error_response = json!({
                    "error": {
                        "code": "INTERNAL_ERROR",
                        "status": 500,
                        "type": "InternalServerError",
                        "message": "An internal server error occurred",
                        "details": {
                            // En producción podrías no incluir el mensaje real
                            "debug_info": msg,
                            "should_retry": true
                        },
                        "timestamp": timestamp
                    }
                });

                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }

            ConfigError::NotFound(msg) => {
                let error_response = json!({
                    "error": {
                        "code": "INTERNAL_ERROR",
                        "status": 500,
                        "type": "InternalServerError",
                        "message": "An internal server error occurred",
                        "details": {
                            // En producción podrías no incluir el mensaje real
                            "debug_info": msg,
                            "should_retry": true
                        },
                        "timestamp": timestamp
                    }
                });
                (StatusCode::NOT_FOUND, Json(error_response)).into_response()
            }
        }
    }
}
