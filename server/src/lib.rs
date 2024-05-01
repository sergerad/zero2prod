use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use std::net::TcpListener;

pub mod configuration;
pub mod routes;

pub fn run(listener: TcpListener, pool: sqlx::PgPool) -> anyhow::Result<Server> {
    // Enable logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create copyable reference to pool
    let pool = web::Data::new(pool);

    // Run server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
