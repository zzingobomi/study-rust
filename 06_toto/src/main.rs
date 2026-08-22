use actix_web::{App, HttpResponse, HttpResponseBuilder, HttpServer, get, http::StatusCode, web};
use askama::{Error, Template};

fn into_response<T: askama::Template>(tmpl: &T) -> HttpResponse {
    try_into_response(tmpl).unwrap_or_else(|err| HttpResponse::from_error(err.into_io_error()))
}

fn try_into_response<T: askama::Template>(tmpl: &T) -> Result<HttpResponse, Error> {
    let value = tmpl.render()?;
    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("text/html; charset=UTF-8")
        .body(value))
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    tasks: Vec<String>,
}

#[get("/")]
async fn todo() -> HttpResponse {
    let tasks = vec![
        "작업1".to_string(),
        "작업2".to_string(),
        "작업3".to_string(),
    ];
    let todo = TodoTemplate { tasks };
    into_response(&todo)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse {
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    into_response(&hello)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(todo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
