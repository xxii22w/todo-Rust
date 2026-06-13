use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_prometheus::metrics::counter;
use tracing::info;

use crate::{
    AppState,
    handlers::{
        auth::models::{LoginRequest, LoginResponse},
        models::{ErrorResponse, JsonResponse},
    },
    service::auth::Error,
};

pub async fn header(
    State(AppState { auth_service, .. }): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    info!("Login request: {request:?}");

    match auth_service.login(request).await {
        Ok(token) => (
            StatusCode::OK,
            Json(JsonResponse::Success(LoginResponse { token })),
        ),
        Err(error) => {
            if matches!(error, Error::InvalidPassword) {
                counter!("login_invalid_password").increment(1);
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(JsonResponse::Error(ErrorResponse::from_error(error))),
                );
            }

            if let Error::Database(sqlx::Error::RowNotFound) = error {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(JsonResponse::Error(ErrorResponse::from_str(
                        "Invalid username!",
                    ))),
                );
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonResponse::Error(ErrorResponse::from_error(error))),
            )
        }
    }
}
