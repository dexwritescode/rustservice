use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, routing::get, routing::post, Json, Router};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::sync::Arc;

use crate::{
    database::AppState,
    models::{NewTodo, Todo},
    schema::todos,
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        .route("/todo", post(create_todo))
        .with_state(Arc::new(state))
}

async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;

    let res = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;

    let todo = todos::dsl::todos
        .find(todo_id)
        .select(Todo::as_select())
        .first(&mut conn)
        .optional();

    match todo {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(not_found_error(&format!("Unable to find todo {}", todo_id))),
        Err(e) => Err(internal_error(e)),
    }
}

// Map any error into a `500 Internal Server Error`
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Map any error into a `404 Internal Server Error`
fn not_found_error(msg: &str) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, msg.to_string())
}
