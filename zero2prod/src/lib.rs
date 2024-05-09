use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;
