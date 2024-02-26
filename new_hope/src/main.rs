use std::{net::SocketAddr, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;



#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    vorname: String,
    nachname: String,
    rights: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

type SharedUsersList = Arc<Mutex<Vec<User>>>;


async fn create_user(Json(friend): Json<User>, state: Extension<SharedUsersList>) -> impl IntoResponse {
    let mut friends = state.0.lock().await;

    friends.push(friend);
    (StatusCode::CREATED, Json(()))
}

async fn get_all_users(state: Extension<SharedUsersList>) -> impl IntoResponse {
    let friends = state.0.lock().await;
    (StatusCode::OK, Json(friends.clone()))
}


async fn hello_world() -> &'static str {
    "Hello, World!"
}

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(hello_world));

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }


#[tokio::main]
async fn main() {

    let friends_list = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/user/new", post(create_user))
        .route("/users", get(get_all_users.clone()));
        // .route("/friends/{id}", put(update_friend))
        // .route("/friends/{id}", delete(delete_friend))

    let addr = SocketAddr::from(([0,  0,  0,  0],  3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}