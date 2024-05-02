#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let command = std::env::args().nth(1);
    match command {
        Some(cmd) if cmd == "migrate" => {
            pg::migrate_pg().await?;
            Ok(())
        }
        Some(cmd) if cmd == "run" => {
            pg::spawn_and_wait().await?;
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Usage: pg [migrate|run]")),
    }
}
