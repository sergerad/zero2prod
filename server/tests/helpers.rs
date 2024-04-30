use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap().to_string();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://{addr}")
}
