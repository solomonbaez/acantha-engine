use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("index.html"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/game", web::get().to(game))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
