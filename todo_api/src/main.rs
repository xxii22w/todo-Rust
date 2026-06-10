use std::sync::Arc;

use axum::{
    Router,
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, patch, post, put},
};

use crate::db::{connection_pool, migration::migrate_db};

mod db;
mod handlers;
mod service;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<service::todo::Service>,
    pub auth_service: Arc<service::auth::Service>,
    pub jwt_service: Arc<service::jwt::Service>,
}

async fn auth_middleware(
    State(AppState { jwt_service, .. }): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) {
        let auth_header_content = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
        if !auth_header_content.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let auth_token = auth_header_content.replace("Bearer ", "");
        let context_user = jwt_service
            .verify_token(auth_token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        req.extensions_mut().insert(context_user);

        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    migrate_db()?;

    let db_conn_pool = connection_pool()?;
    let todo_service = Arc::new(service::todo::Service::new(db_conn_pool.clone())?);
    let jwt_service = Arc::new(service::jwt::Service::new()?);
    let auth_service = Arc::new(service::auth::Service::new(
        jwt_service.clone(),
        db_conn_pool.clone(),
    )?);

    let app_state = AppState {
        todo_service,
        auth_service,
        jwt_service: jwt_service.clone(),
    };

    let router = Router::new()
        .route(
            "/todo",
            get(handlers::todo::lists::handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/todo/{id}",
            get(handlers::todo::get::handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/todo",
            post(handlers::todo::create::handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/todo/{id}",
            delete(handlers::todo::delete::handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/todo/{id}",
            patch(handlers::todo::partial_update::handler).route_layer(
                middleware::from_fn_with_state(app_state.clone(), auth_middleware),
            ),
        )
        .route(
            "/todo/{id}",
            put(handlers::todo::update::handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/auth/register",
            post(handlers::auth::registration::handler),
        )
        .route("/auth/login", post(handlers::auth::login::header))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
