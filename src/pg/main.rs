use std::{fs, sync::mpsc::channel};

use zero2prod::{configuration::get_configuration, pg::run_pg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read settings from config file
    let settings = get_configuration()?.database;

    // Start Postgres container
    let (node, _) = run_pg(&settings).await?;
    println!("Container is running. Waiting for signal to stop.");

    // Store connection string in .env file
    let connection_string = settings.connection_string(node.get_host_port_ipv4(5432).await);
    fs::write(".env", format!("DATABASE_URL=\"{connection_string}\"",))?;
    println!("Connection string written to .env");

    // Listen for signal to stop
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))?;
    rx.recv()?;

    // Shut container down
    println!("Shutting down");
    node.stop().await;
    Ok(())
}
