use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().rout("/", get(|| async { "Hello World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

}

let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));


async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
