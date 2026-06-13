use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum JsonResponse<T: Serialize> {
    Success(T),
    Error(ErrorResponse),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn from_error(error: impl std::error::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }

    pub fn from_str(error: &str) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
