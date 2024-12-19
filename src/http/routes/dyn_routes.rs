use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    http::handlers::dyn_handlers,
    state::dyn_state::{AppStateDyn, InMemoryUserRepo},
};

pub fn routes(user_repo: &InMemoryUserRepo) -> Router {
    Router::new()
        .route("/users/:id", get(dyn_handlers::get_user_dyn))
        .route("/users", post(dyn_handlers::create_user_dyn))
        .route("/users", get(dyn_handlers::list_users_dyn))
        .with_state(AppStateDyn {
            user_repo: Arc::new(user_repo.clone()),
        })
}
