use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod trace;

pub fn run(listener: TcpListener, pool: sqlx::PgPool) -> anyhow::Result<Server> {
    // Create copyable reference to pool
    let pool = web::Data::new(pool);

    // Run server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .route("/health", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
