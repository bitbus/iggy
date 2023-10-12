use crate::http::error::CustomError;
use crate::http::jwt::Identity;
use crate::http::mapper;
use crate::http::state::AppState;
use crate::streaming::session::Session;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use iggy::models::identity_info::IdentityInfo;
use iggy::models::pat::{PersonalAccessTokenInfo, RawPersonalAccessToken};
use iggy::users::create_pat::CreatePersonalAccessToken;
use iggy::users::login_pat::LoginWithPersonalAccessToken;
use iggy::validatable::Validatable;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_pats).post(create_pat))
        .route("/:name", delete(delete_pat))
        .route("/login", post(login_with_pat))
        .with_state(state)
}

async fn get_pats(
    State(state): State<Arc<AppState>>,
    Extension(identity): Extension<Identity>,
) -> Result<Json<Vec<PersonalAccessTokenInfo>>, CustomError> {
    let system = state.system.read().await;
    let pats = system
        .get_personal_access_tokens(&Session::stateless(identity.user_id))
        .await?;
    let pats = mapper::map_pats(&pats);
    Ok(Json(pats))
}

async fn create_pat(
    State(state): State<Arc<AppState>>,
    Extension(identity): Extension<Identity>,
    Json(command): Json<CreatePersonalAccessToken>,
) -> Result<Json<RawPersonalAccessToken>, CustomError> {
    command.validate()?;
    let system = state.system.read().await;
    let token = system
        .create_personal_access_token(
            &Session::stateless(identity.user_id),
            &command.name,
            command.expiry,
        )
        .await?;
    Ok(Json(RawPersonalAccessToken { token }))
}

async fn delete_pat(
    State(state): State<Arc<AppState>>,
    Extension(identity): Extension<Identity>,
    Path(name): Path<String>,
) -> Result<StatusCode, CustomError> {
    let system = state.system.read().await;
    system
        .delete_personal_access_token(&Session::stateless(identity.user_id), &name)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn login_with_pat(
    State(state): State<Arc<AppState>>,
    Json(command): Json<LoginWithPersonalAccessToken>,
) -> Result<Json<IdentityInfo>, CustomError> {
    command.validate()?;
    let system = state.system.read().await;
    let user = system
        .login_with_personal_access_token(&command.token, None)
        .await?;
    Ok(Json(IdentityInfo {
        user_id: user.id,
        token: Some(state.jwt_manager.generate(user.id)),
    }))
}