use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use parking_lot::lock_api::Mutex;

use crate::{
    http::handlers::generic_handlers,
    state::{dyn_state::InMemoryUserRepo, generic_state::AppStateGeneric},
};

pub fn routes(user_repo: &InMemoryUserRepo) -> Router {
    Router::new()
        .route(
            "/users/:id",
            get(generic_handlers::get_user_generic::<InMemoryUserRepo>),
        )
        .route(
            "/users",
            post(generic_handlers::create_user_generic::<InMemoryUserRepo>),
        )
        .route(
            "/users",
            get(generic_handlers::list_users_generic::<InMemoryUserRepo>),
        )
        .with_state(AppStateGeneric {
            user_repo: user_repo.clone(),
        })
}
