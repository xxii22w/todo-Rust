use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppState,
    handlers::{
        models::{ErrorResponse, JsonResponse},
        todo::models::Todo,
    },
    service::jwt::ContextUser,
};

#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "Successfully listed TODOs", body = JsonResponse<Vec<Todo>>, example = json!({"success": [{"id": 3,"title": "Some title 3","description": "Some description 3"}]})),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn handler(
    State(AppState { todo_service, .. }): State<AppState>,
    Extension(user): Extension<ContextUser>,
) -> impl IntoResponse {
    match todo_service.list(user.user_id as i32).await {
        Ok(result) => (
            StatusCode::OK,
            Json(JsonResponse::Success(
                result
                    .iter()
                    .map(|value| value.into())
                    .collect::<Vec<Todo>>(),
            )),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JsonResponse::Error(ErrorResponse::from_error(error))),
        ),
    }
}
