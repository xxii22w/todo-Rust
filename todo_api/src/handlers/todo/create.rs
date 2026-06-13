use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::info;

use crate::{
    AppState,
    handlers::{
        models::{ErrorResponse, JsonResponse},
        todo::models::{CreateTodoRequest, Todo},
    },
    service::{jwt::ContextUser, todo::Service},
};

pub async fn handler(
    State(AppState { todo_service, .. }): State<AppState>,
    Extension(user): Extension<ContextUser>,
    Json(request): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    info!("Create TODO request: {request:?}");

    match todo_service
        .create(user.user_id as i32, request.into())
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            Json(JsonResponse::Success(Todo::from(result))),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JsonResponse::Error(ErrorResponse::from_error(error))),
        ),
    }
}
