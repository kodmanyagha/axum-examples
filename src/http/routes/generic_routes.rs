use std::sync::Arc;

use axum::{routing::get, Router};
use parking_lot::lock_api::Mutex;

use crate::{
    http::handlers::generic_handlers,
    state::{dyn_state::InMemoryUserRepo, generic_state::AppStateGeneric},
};

pub fn routes(user_repo: InMemoryUserRepo) -> Router {
    Router::new()
        .route(
            "/users/:id",
            get(generic_handlers::get_user_generic::<InMemoryUserRepo>),
        )
        .with_state(AppStateGeneric { user_repo })
}
