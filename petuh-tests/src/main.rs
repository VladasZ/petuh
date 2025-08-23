use anyhow::Result;
use common::initial_setup;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = initial_setup("petuh-tests")?;

    // redis()?.set("some_test_key", "aaa pff")?;

    Ok(())
}
