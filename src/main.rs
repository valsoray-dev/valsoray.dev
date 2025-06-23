use actix_files::{Files, NamedFile};
use actix_web::http::{StatusCode, header::LINK};
use actix_web::{App, Either, HttpRequest, HttpServer, Responder, get, middleware};
use log::info;

#[get("/blog")]
async fn rickroll(req: HttpRequest) -> impl Responder {
    // Serve the rickroll page if the request made by htmx
    if req.headers().get("hx-request").is_some_and(|v| v == "true") {
        return Either::Left(NamedFile::open_async("./assets/rickroll.html").await);
    }

    Either::Right(
        NamedFile::open_async("./assets/404.jpg")
            .await
            .customize()
            .with_status(StatusCode::NOT_FOUND)
            .append_header((LINK, "<https://http.cat/404.jpg>; rel=\"source\"")),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&dotenvy::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    let port: u16 = dotenvy::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Invalid PORT value");

    info!("Server is running at http://localhost:{port}");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::Logger::new("%r [%s] | Time: [%T] | \"%{User-Agent}i\"")
                    .log_level(log::Level::Debug),
            )
            .service(rickroll)
            .service(
                Files::new("/", "./assets")
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
