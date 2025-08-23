#![allow(unreachable_code)]

use tracing::info;

use crate::{
    commands::{Command, handle_command},
    text::handle_text,
};
mod commands;
mod data;
mod llm;
mod phrases;
mod responses;
mod text;
mod weather;
mod yayko;

use anyhow::Result;
use common::{Environment, initial_setup};
use teloxide::prelude::*;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const PETUHI: &[&str] = &["Максим", "Владик", "Владас", "Рома", "Настя", "Алёна", "Витёк"];

fn teloxide_token() -> Result<String> {
    Ok(std::env::var(Environment::select(
        "TELOXIDE_TOKEN_STAGING",
        "TELOXIDE_TOKEN_PRODUCTION",
    ))?)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");

    let _guard = initial_setup("petuh")?;

    info!("Starting Telegram bot...");

    let bot = Bot::new(teloxide_token()?);

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint(handle_command))
        .branch(Update::filter_message().endpoint(handle_text));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn debug() -> Result<()> {
    dotenv::dotenv().ok();
    println!("Starting Telegram bot...");

    let bot = Bot::from_env();

    let chat = bot.get_chat(ChatId(1)).await?;

    dbg!(&chat);

    bot.delete_message(ChatId(1), teloxide::types::MessageId(1)).await?;

    Ok(())
}

#[test]
fn system_info() {
    dbg!(APP_VERSION);
}
