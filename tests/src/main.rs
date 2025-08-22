use anyhow::Result;
use common::{initial_setup, redis};
use redis::TypedCommands;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = initial_setup("tests")?;

    redis()?.set("some_test_key", "aaa pff")?;

    Ok(())
}
