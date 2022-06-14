use anyhow::Error;
use dy::logs::log_init;

#[tokio::main]
async fn main() -> Result<(), Error> {
    log_init();

    dy::cli::cli_run().await?;
    
    Ok(())
}
