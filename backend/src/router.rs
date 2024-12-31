use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::{MySql, Pool};
use tower::ServiceBuilder;

use crate::{
    api::{
        auth::{login_route, logout_route, refresh_token_route, signup_route, verify_token_route},
        cors::configure_cors,
        root::health_checker,
    },
    config::{api_state::ApiState, environment::EnvironmentConfig},
    middlewares::auth_middleware::auth_middleware,
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
        .nest("/protected", protected_routes(state))
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

fn protected_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/teste", get(teste_route))
        .layer(ServiceBuilder::new().layer(from_fn({
            let app_state = state.clone(); // Clone app_state for the async block
            move |req, next| {
                let state = app_state.clone(); // Clone for use in the async block
                async move { auth_middleware(req, next, state).await }
            }
        })))
        .with_state(state)
}

async fn teste_route(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    state.environment.database_url.to_string()
}
