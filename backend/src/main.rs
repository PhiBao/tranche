mod api;
mod config;
mod db;
mod domain;
mod executor;
mod integrations;
mod net;
mod payments;
mod store;

use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub cfg: Arc<config::Config>,
    pub http: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tranche=info,tower_http=info".into()),
        )
        .init();

    let cfg = config::Config::from_env();
    tracing::info!(
        bind = %cfg.bind_addr,
        engines = ?cfg.engine_report(),
        "starting tranche"
    );

    let pool = db::connect(&cfg.database_url, cfg.database_max_connections).await?;

    let http = reqwest::Client::builder()
        .user_agent("tranche-acceptance-agent/0.1")
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;

    let state = AppState {
        pool,
        cfg: Arc::new(cfg.clone()),
        http,
    };

    let api = Router::new()
        .route("/health", get(api::health))
        .route("/programs", get(api::programs))
        .route("/milestones", get(api::milestones).post(api::create_milestone))
        .route("/milestones/preview", post(api::preview_graph))
        .route("/milestones/{id}", get(api::milestone_detail))
        .route("/milestones/{id}/submit", post(api::submit_evidence))
        .route("/milestones/{id}/sample", post(api::submit_sample))
        .route("/submissions/{id}/verify", post(api::verify_submission))
        .route("/payments/{id}/approve", post(api::approve_payment))
        .route("/receipts/{slug}", get(api::receipt_detail));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let mut app = Router::new()
        .nest("/api", api)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    if let Some(frontend_dir) = cfg.frontend_dir.as_ref() {
        if frontend_dir.exists() {
            let index = frontend_dir.join("index.html");
            tracing::info!(dir = %frontend_dir.display(), "serving static frontend");
            app = app.fallback_service(
                ServeDir::new(frontend_dir).not_found_service(ServeFile::new(index)),
            );
        } else {
            tracing::warn!(dir = %frontend_dir.display(), "frontend build not found");
            app = app.route("/", get(root_status));
        }
    } else {
        app = app.route("/", get(root_status));
    }

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("listening on http://{}", cfg.bind_addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn root_status() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "service": "tranche",
        "message": "API is running. Build the frontend with `pnpm build` in frontend/ or run it in dev mode.",
        "api": "/api/health"
    }))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
