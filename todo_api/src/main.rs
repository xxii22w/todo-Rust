use std::{
    arch::x86_64::_mm256_mask_fixupimm_pd,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use axum::{
    Json, Router,
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::{self, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, patch, post, put},
};
use axum_prometheus::PrometheusMetricLayer;
use moka::future::Cache;
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing::{Level, info, warn};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    db::connection_pool,
    handlers::models::{ErrorResponse, JsonResponse},
};
use handlers::todo::__path_list;

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

#[derive(Clone)]
pub struct FixedWindowLimiter {
    // ket是ip,value是当前窗口内的请求计数器
    pub registry: Cache<SocketAddr, Arc<AtomicU64>>,
    pub max_requests: u64,
}

impl FixedWindowLimiter {
    pub fn new(max_requests: u64, window_size: Duration) -> Self {
        Self {
            registry: Cache::builder().time_to_live(window_size).build(),
            max_requests,
        }
    }
}

pub async fn fixed_window_middleware(
    State(limiter): State<FixedWindowLimiter>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<JsonResponse<String>>)> {
    let counter = limiter
        .registry
        .get_with(addr, async { Arc::new(AtomicU64::new(0)) })
        .await;

    let current_requests = counter.fetch_add(1, Ordering::Relaxed) + 1;

    if current_requests > limiter.max_requests {
        warn!(client_ip = %addr,requests = current_requests,"Too many requests have been blocked. Please try again later.");

        let error_info = ErrorResponse::from_str("Too Many Requests");

        let error_response = JsonResponse::Error(error_info);

        return Err((StatusCode::TOO_MANY_REQUESTS, Json(error_response)));
    }

    Ok(next.run(request).await)
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    // 配置文件输出与滚动
    let file_append = tracing_appender::rolling::daily("logs", "todo-api.log");
    // 设置非阻塞写入，防止写操作阻塞主线程
    let (non_blocking_file_write, _guard) = tracing_appender::non_blocking(file_append);
    // 利用and组合器，把标准输出和文件绑定在一起
    let stdout_writer = std::io::stdout.with_max_level(Level::INFO);
    let file_writer = non_blocking_file_write.with_max_level(Level::TRACE);

    let combined_writer = stdout_writer.and(file_writer);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(combined_writer)
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscribe");

    _guard
}

#[derive(Debug, Serialize)]
struct OpenApiModifier;

impl Modify for OpenApiModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "api_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(list),
    modifiers(&OpenApiModifier),
    security(
        ("api_token" = ["edit:items", "read:items"])
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _log_guard = init_tracing();

    let limiter = FixedWindowLimiter::new(5, Duration::from_secs(10));
    let db_conn_pool = connection_pool().await?;
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

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

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
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .layer(prometheus_layer)
        .layer(middleware::from_fn_with_state(
            limiter,
            fixed_window_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    info!("Starting HTTP server at 0.0.0.0:9999...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
