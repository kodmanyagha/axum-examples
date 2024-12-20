mod http;
// mod main_working;
mod state;
mod utils;

use axum::{routing, Router};
use http::{
    handlers::generic_handlers::{create_user_generic, get_user_generic},
    routes::{dyn_routes, generic_routes},
};
use state::{dyn_state::InMemoryUserRepo, generic_state::AppStateGeneric};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // main_working::main_working().await;

    let user_repo = InMemoryUserRepo::new();

    let app = Router::new()
        .nest("/dyn", dyn_routes::routes(&user_repo))
        .nest("/generic", generic_routes::routes(&user_repo));

    let listener = giver!(TcpListener::bind("127.0.0.1:3000").await);

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
