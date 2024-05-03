#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .init();

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
