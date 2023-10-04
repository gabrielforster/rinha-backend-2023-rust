use axum::{
    routing::{get, post},
    Router, response::IntoResponse,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pessoas", post(handle_create))
        .route("/pessoas", get(handle_search))
        .route("/pessoas/:id", get(handle_find))
        .route("/contagem-pessoas", get(handle_count));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_create () -> impl IntoResponse { 
    return "create"
}

async fn handle_search () -> impl IntoResponse {
    return "search/list"
}

async fn handle_find () -> impl IntoResponse {
    return "find"
}

async fn handle_count () -> impl IntoResponse {
    return "count"
}
