use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::repository::{PersistenceError, PostgresRepository};
use crate::types::NewPerson;

mod repository;
mod types;


type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://rinha:rinha@localhost:5432/rinha"));

    let database_pool_size = env::var("DATABASE_POOL")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(30);

    let repo = PostgresRepository::connect(&database_url, database_pool_size).await.unwrap();
    let app_state = Arc::new(repo);

    let app = Router::new()
        .route("/pessoas", get(search))
        .route("/pessoas/:id", get(find))
        .route("/pessoas", post(create))
        .route("/contagem-pessoas", get(count))
        .with_state(app_state);

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct PersonSearchQuery {
    #[serde(rename = "t", default)]
    query: String,
}
async fn search(
    State(people): State<AppState>,
    Query(PersonSearchQuery { query }): Query<PersonSearchQuery>,
) -> impl IntoResponse {
    match people.search_people(&query).await {
        Ok(people) => Ok(Json(people)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn find(
    State(people): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match people.find_person(id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create(
    State(people): State<AppState>,
    Json(person): Json<NewPerson>,
) -> impl IntoResponse {
    match people.create_person(person).await {
        Ok(id) => Ok((
                StatusCode::CREATED,
                [(header::LOCATION, format!("/pessoas/{}", id))],
                )),
        Err(PersistenceError::UniqueViolation) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn count(State(people): State<AppState>) -> impl IntoResponse {
    match people.count_people().await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
