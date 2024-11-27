use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use validator::ValidationErrors;
use log::error;

/// Custom error types for the API
#[derive(Error, Debug)]
pub enum ApiError {
    /// Validation error for input parameters
    #[error("validation_error: {0}")]
    ValidationError(#[from] ValidationErrors),
    
    /// Internal server error
    #[error("internal_error: {0}")]
    InternalError(String),

    /// Calculation error
    #[error("calculation_error: {0}")]
    CalculationError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(errors) => {
                error!("Validation error: {:?}", errors);
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "validation_error",
                    "details": errors
                }))
            }
            ApiError::InternalError(message) => {
                error!("Internal server error: {}", message);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": message
                }))
            }
            ApiError::CalculationError(message) => {
                error!("Calculation error: {}", message);
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": "calculation_error",
                    "message": message
                }))
            }
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::CalculationError(_) => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationError;

    #[test]
    fn test_validation_error_response() {
        let mut errors = ValidationErrors::new();
        let mut error = ValidationError::new("test");
        error.message = Some("test message".into());
        errors.add("field", error);

        let api_error = ApiError::ValidationError(errors);
        let response = api_error.error_response();
        
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_internal_error_response() {
        let api_error = ApiError::InternalError("test error".to_string());
        let response = api_error.error_response();
        
        assert_eq!(response.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_calculation_error_response() {
        let api_error = ApiError::CalculationError("division by zero".to_string());
        let response = api_error.error_response();
        
        assert_eq!(response.status(), actix_web::http::StatusCode::UNPROCESSABLE_ENTITY);
    }
}
