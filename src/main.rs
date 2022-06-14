use anyhow::Error;
use dy::logs::log_init;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dy::cli::cli_run().await?;
    log_init();
    Ok(())
}
