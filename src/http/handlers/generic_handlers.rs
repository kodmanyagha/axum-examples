use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::state::{
    dyn_state::{PaginationInfo, PaginationParam, User, UserCreateParams, UserRepo},
    generic_state::AppStateGeneric,
};

pub async fn list_users_generic<T>(
    State(state): State<AppStateGeneric<T>>,
    Query(pagination): Query<PaginationParam>,
) -> Json<PaginationInfo<User>>
where
    T: UserRepo,
{
    Json(state.user_repo.paginate(pagination.page, pagination.rpp))
}

pub async fn create_user_generic<T>(
    State(state): State<AppStateGeneric<T>>,
    Json(params): Json<UserCreateParams>,
) -> Json<User>
where
    T: UserRepo,
{
    let user = User {
        id: state.user_repo.get_current_id(),
        name: params.name,
    };

    state.user_repo.save(&user);

    Json(user)
}

/**
 * Axum gives complex and misunderstandable errors, for simplifying them you can add this
 * macro on top of the handlers. Details here:
 * https://users.rust-lang.org/t/in-axum-im-getting-an-error-about-generics-please-help/122741
 */
// #[axum_macros::debug_handler]
pub async fn get_user_generic<T>(
    State(state): State<AppStateGeneric<T>>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode>
where
    T: UserRepo,
{
    match state.user_repo.get_by_id(id) {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
