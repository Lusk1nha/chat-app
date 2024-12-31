use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{MySql, Pool};

use crate::{
    api::{
        auth::{login_route, logout_route, refresh_token_route, signup_route, verify_token_route},
        cors::configure_cors,
        root::health_checker,
    },
    config::{api_state::ApiState, environment::EnvironmentConfig},
    path::{API_PATH, AUTH_PATH, ROOT_PATH},
};

pub async fn generate_routes(config: &EnvironmentConfig, pool: &Pool<MySql>) -> Router {
    let state = Arc::new(ApiState::new(pool.clone(), config.clone()));
    let cors = configure_cors();

    let api_routes = api_routes(state.clone());

    Router::new().nest(API_PATH, api_routes).layer(cors)
}

fn api_routes(state: Arc<ApiState>) -> Router {
    let auth_routes = auth_routes(state.clone());

    Router::new()
        .route(ROOT_PATH, get(health_checker))
        .nest(AUTH_PATH, auth_routes)
}

fn auth_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/login", post(login_route))
        .route("/signup", post(signup_route))
        .route("/logout", post(logout_route))
        .route("/refresh", post(refresh_token_route))
        .route("/verify", post(verify_token_route))
        .with_state(state)
}
