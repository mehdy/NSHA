use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::env;

mod post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(post::create)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
