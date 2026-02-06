use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, fmt};

use crate::app::config::config;
use crate::app::middleware::trace;
use crate::app::state::AppState;

#[tokio::main]
pub async fn start() {
    let config = config::AppConfig::load().expect("failed to load config");

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .compact()
        .with_ansi(true)
        .with_target(false)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = crate::infra::database::setup::init(
        &config.database.url,
        config.database.max_connections,
        config.database.connect_timeout_secs,
    )
    .await
    .expect("failed to setup database");

    let state = AppState {
        db,
        config: config.clone(),
    };

    let app = crate::app::routes::router()
        .layer(trace::global_trace_layer())
        .with_state(state);

    let addr = SocketAddr::new(config.app.host.parse().unwrap(), config.app.port);

    info!("server is running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
