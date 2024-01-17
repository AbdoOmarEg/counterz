use askama::Template;
use askama_axum::IntoResponse;
// use axum::response::Html;
use axum::Form;
use axum::{routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("hello");
    // build our application with a route
    let app = Router::new().route("/", get(base).post(increment));
    // .route("/ok", get(show_form).post(accept_form));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

static mut COUNTERZ: i32 = 0;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {
    title: String,
    count: i32,
}

async fn base() -> impl IntoResponse {
    let base_template = BaseTemplate {
        title: String::from("helloz base_template"),
        count: unsafe { COUNTERZ },
    };

    base_template.into_response()
}

#[derive(Template)]
#[template(path = "increment.html")]
struct IncrementTemplate {
    title: String,
    count: i32,
}

#[derive(Deserialize, Debug)]
struct CurrentCount {
    value: i32,
}

async fn increment(Form(current_count): Form<CurrentCount>) -> impl IntoResponse {
    println!("hello");
    dbg!(&current_count);
    unsafe { COUNTERZ += 1 };
    let increment_template = IncrementTemplate {
        title: String::from("helloz from incrementz"),
        count: unsafe { COUNTERZ },
    };
    increment_template.into_response()
}
//
// async fn show_form() -> Html<&'static str> {
//     Html(
//         r#"
//         <!doctype html>
//         <html>
//             <head></head>
//             <body>
//                 <form action="/ok" method="post">
//                     <label for="name">
//                         Enter your name:
//                         <input type="text" name="name">
//                     </label>
//
//                     <label>
//                         Enter your email:
//                         <input type="text" name="email">
//                     </label>
//
//                     <input type="submit" value="Subscribe!">
//                 </form>
//             </body>
//         </html>
//         "#,
//     )
// }
//
// #[derive(Deserialize, Debug)]
// #[allow(dead_code)]
// struct Input {
//     name: String,
//     email: String,
// }
//
// async fn accept_form(Form(input): Form<Input>) {
//     dbg!(&input);
// }
