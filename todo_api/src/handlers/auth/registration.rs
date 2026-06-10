use axum::{
    Json,
    extract::State,
    http::{StatusCode, request},
    response::IntoResponse,
};

use crate::{
    AppState,
    handlers::{
        auth::models::RegistrationRequest,
        models::{ErrorResponse, JsonResponse},
    },
};

pub async fn handler(
    State(AppState { auth_service, .. }): State<AppState>,
    Json(request): Json<RegistrationRequest>,
) -> impl IntoResponse {
    println!("Registration Request: {request:?}");

    match auth_service.register(request) {
        Ok(_) => (StatusCode::OK, Json(JsonResponse::Success(true))),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JsonResponse::Error(ErrorResponse::from_error(error))),
        ),
    }
}
