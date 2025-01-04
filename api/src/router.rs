use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::{MySql, Pool};
use tower::ServiceBuilder;

use crate::{
    api::{
        auth::{
            login_route, logout_route, refresh_token_route, signup_route, validate_session_route,
            verify_token_route,
        },
        cors::configure_cors,
        profile::{
            create_profile_route, delete_profile_route, find_current_profile_route,
            find_profile_by_id_route, find_profile_by_user_id_route, update_profile_route,
        },
        root::health_checker,
    },
    config::{api_state::ApiState, environment::EnvironmentConfig},
    middlewares::auth_middleware::auth_middleware,
    path::{API_PATH, AUTH_PATH, PROFILE_PATH, ROOT_PATH},
};

pub async fn generate_routes(config: &EnvironmentConfig, pool: &Pool<MySql>) -> Router {
    let state = Arc::new(ApiState::new(pool.clone(), config.clone()));
    let cors = configure_cors();

    let api_routes = api_routes(state.clone());

    Router::new().nest(API_PATH, api_routes).layer(cors)
}

fn api_routes(state: Arc<ApiState>) -> Router {
    let auth_routes = auth_routes(state.clone());
    let profile_routes = profile_routes(state.clone());
    let protected_routes = protected_routes(state.clone());

    println!("{:?}", profile_routes);

    Router::new()
        .route(ROOT_PATH, get(health_checker))
        .nest(AUTH_PATH, auth_routes)
        .merge(protected_routes)
}

fn auth_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/login", post(login_route))
        .route("/signup", post(signup_route))
        .route("/logout", post(logout_route))
        .route("/refresh", post(refresh_token_route))
        .route("/verify", post(verify_token_route))
        .route("/validate-session", post(validate_session_route))
        .with_state(state)
}

fn protected_routes(state: Arc<ApiState>) -> Router {
    let profile_routes = profile_routes(state.clone());

    Router::new()
        .nest(PROFILE_PATH, profile_routes)
        .layer(ServiceBuilder::new().layer(from_fn({
            let app_state = state.clone(); // Clone app_state for the async block
            move |req, next| {
                let state = app_state.clone(); // Clone for use in the async block
                async move { auth_middleware(req, next, state).await }
            }
        })))
}

fn profile_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/user/:user_id", get(find_profile_by_user_id_route))
        .route("/:id", get(find_profile_by_id_route))
        .route("/currentUser", get(find_current_profile_route))
        .route("/", post(create_profile_route))
        .route("/", patch(update_profile_route))
        .route("/", delete(delete_profile_route))
        .with_state(state)
}

async fn teste_route(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    state.environment.database_url.to_string()
}
