use std::{ops::Deref, sync::Arc};

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::info;

use crate::{
    AppState,
    handlers::models::{ErrorResponse, JsonResponse},
    service::{self, jwt::ContextUser, todo::Service},
};

pub async fn handler(
    State(AppState { todo_service, .. }): State<AppState>,
    Extension(user): Extension<ContextUser>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    info!("Delete TODO handler request id: {id}");

    match todo_service.delete(user.user_id as i32, id as i32).await {
        Ok(_) => (StatusCode::OK, Json(JsonResponse::Success(true))),
        Err(error) => {
            if matches!(
                error,
                service::todo::Error::Database(sqlx::Error::RowNotFound)
            ) {
                return (
                    StatusCode::NOT_FOUND,
                    Json(JsonResponse::Error(ErrorResponse::from_str(
                        "TODO not found!",
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
