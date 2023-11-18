use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, routing::delete, routing::get, routing::post, Json, Router};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::schema::todos::id;
use crate::{
    database::AppState,
    models::{NewTodo, Todo},
    schema::todos,
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        .route("/todo/:todo_id", delete(delete_todo))
        .route("/todo", post(create_todo))
        .route("/todo", get(get_all_todos))
        .route("/todo/random", post(create_random_todo))
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http())
}

async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;

    tracing::info!("Creating Todo {:?}", &new_todo);

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

    tracing::info!("Retrieving Todo record from the db. id: {}", &todo_id);

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

async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let conn = &mut state.pool.get().map_err(internal_error)?;

    tracing::info!("Deleting Todo record from the db. id: {}", &todo_id);

    let num_deleted = diesel::delete(todos::dsl::todos.filter(id.eq(todo_id)))
        .execute(conn)
        .map_err(internal_error)?;

    tracing::info!("Deleted {} Todos", num_deleted);

    Ok(())
}

async fn get_all_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;

    tracing::info!("Retrieving all Todos from the db.");

    let todos = todos::dsl::todos
        .select(Todo::as_select())
        .load(&mut conn)
        .optional();

    match todos {
        Ok(Some(todos)) => Ok(Json(todos)),
        Ok(None) => Err(not_found_error("No records found.")),
        Err(e) => Err(internal_error(e)),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Activity {
    pub activity: String,
    #[serde(alias = "type")]
    pub activity_type: String,
}

async fn create_random_todo(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let random_activity: Activity = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .map_err(internal_error)?
        .json()
        .await
        .map_err(internal_error)?;

    tracing::info!("Got: {:?}", random_activity);

    let new_todo = NewTodo {
        title: random_activity.activity,
        body: random_activity.activity_type,
    };

    let mut conn = state.pool.get().map_err(internal_error)?;

    tracing::info!("Creating random Todo {:?}", &new_todo);

    let res = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

// Map any error into a `500 Internal Server Error`
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Map str into a `404 Internal Server Error`
fn not_found_error(msg: &str) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, msg.to_string())
}
