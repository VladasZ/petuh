use anyhow::Result;
use common::{Redis, initial_setup};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = initial_setup("petuh-tests")?;

    info!("Started petuh-test");

    Redis::set("sobaka", "cat")?;

    Ok(())
}
