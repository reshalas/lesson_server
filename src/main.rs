use actix_cors::Cors;
use actix_web::{App, HttpServer};
use handlers::*;
use sqlx::PgPool;

mod handlers;
pub mod repositoryes;
pub mod types;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    repositoryes::init(setup_db().await);
    let port = str::parse(dotenvy::var("PORT").unwrap().as_str()).unwrap();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .wrap(cors)
            .service(build_user_scope())
            .service(build_tasks_scope())
            .service(build_transactions_scope())
            .service(build_slots_scope())
            .service(build_email_verefication_scope())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

async fn setup_db() -> PgPool {
    sqlx::PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("Can't connect to db")
}
