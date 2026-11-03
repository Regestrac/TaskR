use axum::{Router, routing::get};

use crate::handlers::tasks;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(tasks::health))
        .route("/tasks", get(tasks::list).post(tasks::create))
        .route(
            "/tasks/{id}",
            get(tasks::get_one)
                .patch(tasks::update)
                .delete(tasks::delete),
        )
        .with_state(state)
}
