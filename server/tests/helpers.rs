use std::net::TcpListener;

pub struct TestApp {
    pub base_url: String,
    pub pool: sqlx::PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let address = listener.local_addr().unwrap().to_string();
    let connection_string = pg::connection_string("../.env").expect("Failed to read .env file");
    let pool = pg::get_pool(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let server = server::run(listener, pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        base_url: format!("http://{}", address),
        pool,
    }
}
