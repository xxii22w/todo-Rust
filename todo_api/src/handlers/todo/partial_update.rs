use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{StatusCode, request},
    response::IntoResponse,
};

use crate::{
    AppState,
    handlers::{
        models::{ErrorResponse, JsonResponse},
        todo::models::{PartialUpdateTodoRequest, Todo},
    },
    service::{self, jwt::ContextUser, todo::Service},
};

pub async fn handler(
    State(AppState { todo_service, .. }): State<AppState>,
    Extension(user): Extension<ContextUser>,
    Path(id): Path<u64>,
    Json(request): Json<PartialUpdateTodoRequest>,
) -> impl IntoResponse {
    println!("Partiial updatte TODO request: {request:?}");

    match todo_service.partial_update(user.user_id as i32, id as i32, request.into()) {
        Ok(_) => (StatusCode::OK, Json(JsonResponse::Success(true))),
        Err(error) => {
            if matches!(
                error,
                service::todo::Error::Diesel(diesel::result::Error::NotFound)
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
