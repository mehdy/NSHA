#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use env_logger;

mod post;
mod schema;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPC = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(db_url))
        .expect("Failed to create db pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(post::create)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
