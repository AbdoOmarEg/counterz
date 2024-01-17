use askama::Template;
use askama_axum::IntoResponse;
use axum::Form;
use axum::{routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("hello");
    let app = Router::new().route("/", get(base).post(increment));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {
    title: String,
}

async fn base() -> impl IntoResponse {
    let base_template = BaseTemplate {
        title: String::from("helloz base_template"),
    };

    base_template.into_response()
}

#[derive(Template)]
#[template(path = "increment.html")]
struct IncrementTemplate {
    count: i32,
}

#[derive(Deserialize, Debug)]
struct GetCurrentCount {
    value: i32,
}

async fn increment(Form(get_current_count): Form<GetCurrentCount>) -> impl IntoResponse {
    println!("hello");
    dbg!(&get_current_count);
    let count = get_current_count.value + 1;
    let increment_template = IncrementTemplate { count };
    increment_template.into_response()
}
