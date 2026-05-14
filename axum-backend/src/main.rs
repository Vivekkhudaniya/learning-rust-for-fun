use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};

// ── Data model ────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

// GET /
async fn root() -> &'static str {
    "Server is running!"
}

// GET /users
async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: String::from("Vivek"), email: String::from("vivek@gmail.com") },
        User { id: 2, name: String::from("Raj"),   email: String::from("raj@gmail.com") },
    ];
    Json(users)
}

// GET /users/:id
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    let user = User {
        id,
        name: String::from("Vivek"),
        email: String::from("vivek@gmail.com"),
    };
    Json(user)
}

// POST /users
async fn create_user(Json(body): Json<CreateUser>) -> Json<User> {
    let new_user = User {
        id: 3,
        name: body.name,
        email: body.email,
    };
    Json(new_user)
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",            get(root))
        .route("/users",       get(get_users))
        .route("/users/:id",   get(get_user))
        .route("/users",       post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
