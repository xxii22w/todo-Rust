use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

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
    println!("Login request: {request:?}");

    match auth_service.login(request) {
        Ok(token) => (
            StatusCode::OK,
            Json(JsonResponse::Success(LoginResponse { token })),
        ),
        Err(error) => {
            if matches!(error, Error::InvalidPassword) {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(JsonResponse::Error(ErrorResponse::from_error(error))),
                );
            }

            if matches!(error, Error::Diesel(diesel::result::Error::NotFound)) {
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
