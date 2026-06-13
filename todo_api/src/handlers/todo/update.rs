use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{StatusCode, request},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    AppState,
    handlers::{
        models::{ErrorResponse, JsonResponse},
        todo::models::{Todo, UpdateTodoRequest},
    },
    service::{self, jwt::ContextUser, todo::Service},
};

pub async fn handler(
    State(AppState { todo_service, .. }): State<AppState>,
    Extension(user): Extension<ContextUser>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    info!("Update TODO request: {request:?}");

    match todo_service
        .update(user.user_id as i32, id as i32, request.into())
        .await
    {
        Ok(_) => (StatusCode::OK, Json(JsonResponse::Success(true))),
        Err(error) => {
            if matches!(
                error,
                service::todo::Error::Database(sqlx::Error::RowNotFound)
            ) {
                return (
                    StatusCode::NOT_FOUND,
                    Json(JsonResponse::Error(ErrorResponse::from_str(
                        "TODO NOT FOUND",
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
