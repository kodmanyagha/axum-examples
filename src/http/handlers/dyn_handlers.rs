use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::state::dyn_state::{
    AppStateDyn, PaginationInfo, PaginationParam, User, UserCreateParams,
};

pub async fn list_users_dyn(
    State(state): State<AppStateDyn>,
    Query(pagination): Query<PaginationParam>,
) -> Json<PaginationInfo<User>> {
    Json(state.user_repo.paginate(pagination.page, pagination.rpp))
}

pub async fn get_user_dyn(
    State(state): State<AppStateDyn>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    match state.user_repo.get_by_id(id) {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_user_dyn(
    State(state): State<AppStateDyn>,
    Json(params): Json<UserCreateParams>,
) -> Json<User> {
    let user = User {
        id: state.user_repo.get_current_id(),
        name: params.name,
    };

    state.user_repo.save(&user);

    Json(user)
}
