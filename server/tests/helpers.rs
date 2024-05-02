use std::net::TcpListener;

use sqlx::{Connection, Executor};

pub struct TestApp {
    pub base_url: String,
    pub db_name: String,
    pub pool: sqlx::PgPool,
}

pub async fn spawn_app() -> TestApp {
    // Create database
    let connection_string = pg::connection_string("../.env").expect("Failed to read .env file");
    let mut connection = sqlx::PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let db_name = uuid::Uuid::new_v4().to_string();
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database {db_name}");

    // Create pool and execute migration
    let connection_string =
        pg::replace_db(connection_string, &db_name).expect("Failed to read .env file");
    let pool = pg::get_pool(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("../db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Server
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let address = listener.local_addr().unwrap().to_string();
    let server = server::run(listener, pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    // App
    TestApp {
        base_url: format!("http://{}", address),
        db_name,
        pool,
    }
}
