mod chat_gpt;

use crate::chat_gpt::query_petuh;
use anyhow::Result;
use rand::prelude::SliceRandom;
use reqwest::Client;
use serde::Deserialize;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::types::{MediaKind, Message, MessageKind};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "🐓 Петушиные команды:")]
enum Command {
    #[command(description = "Помощь петушары.")]
    Help,
    #[command(description = "Приветствие.")]
    K,
    #[command(description = "Уважение Максиму.")]
    M,
    #[command(description = "Уважение Роме.")]
    R,
    #[command(description = "Доброе утро.")]
    Gm,
    #[command(description = "Узнать кто петух.")]
    Kto,
    #[command(description = "Бросить петушиный кубик.")]
    Kub,
    Vladik,
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    dbg!(&msg);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::K => {
            bot.send_message(
                msg.chat.id,
                "Кукарекуууу я петушара!!! Я кукорекою як пятух 🐓. Кок",
            )
            .await?;
        }
        Command::M => {
            bot.send_message(msg.chat.id, "Максим Пятушара!!!! 🐓🐓🐓🐓🐓")
                .await?;
        }
        Command::R => {
            bot.send_message(msg.chat.id, "Рома каблук петушиный 👠")
                .await?;
        }
        Command::Gm => {
            bot.send_message(msg.chat.id, "Доброе утро петушары ебаные! 🐓")
                .await?;
        }
        Command::Kto => {
            let names = [
                "Максим",
                "Владик",
                "Владас",
                "Рома",
                "Настя",
                "Денис",
                "Витя",
            ];
            let name = names.choose(&mut rand::thread_rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
        Command::Kub => {
            bot.send_dice(msg.chat.id).await?;
        }
        Command::Vladik => {
            let user_id = 795896962; // Replace with actual user ID
            let message = format!(
                "Это великий пятушара - <a href=\"tg://user?id={}\">Пятух!!!</a>!",
                user_id
            );

            bot.send_message(msg.chat.id, message)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}

async fn handle_text(bot: Bot, msg: Message) -> ResponseResult<()> {
    dbg!(&msg);

    match msg.kind {
        MessageKind::Common(ref common_message) => match &common_message.media_kind {
            MediaKind::Sticker(sticker) => {
                if sticker.sticker.file.unique_id == "AgADl14AAqISEEs" {
                    bot.send_message(msg.chat.id, "@maxon8871 !! Ну ты и петух!!!")
                        .await?;
                }
            }
            _ => (),
        },
        _ => (),
    }

    if let Some(text) = msg.text() {
        let text = text.to_lowercase();

        if text.starts_with("пятух, ") {
            let text = &text["пятух, ".len()..];

            fn escape_markdown_v2(input: &str) -> String {
                let special_chars = [
                    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}',
                    '.', '!',
                ];
                let mut result = String::new();

                for c in input.chars() {
                    if special_chars.contains(&c) {
                        result.push('\\');
                    }
                    result.push(c);
                }

                result
            }

            bot.send_message(msg.chat.id, query_petuh(&text).await.unwrap())
                .await?;
        }

        if text.contains("--version") || text.contains("-v") {
            bot.send_message(msg.chat.id, "Пятушара 0.2.4").await?;
        }

        if text.contains("погода") {
            let weather = get_weather(&text).await.unwrap();
            bot.send_message(msg.chat.id, weather).await?;
        }

        if text == "Кто петух?" {
            let names = [
                "Максим",
                "Владик",
                "Владас",
                "Рома",
                "Настя",
                "Денис",
                "Витя",
            ];
            let name = names.choose(&mut rand::thread_rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    dotenv::dotenv().ok();
    log::info!("Starting Telegram bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(handle_command),
        )
        .branch(Update::filter_message().endpoint(handle_text));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

async fn get_weather(query: &str) -> Result<String> {
    let city = query.replace('?', "");
    let city = city.split(' ').last().unwrap();

    dbg!(&city);

    dotenv::dotenv()?;
    // let city = text
    //     .split_whitespace()
    //     .nth(1)
    //     .unwrap_or("Москва"); // Пользователь может написать "/погода СПБ"

    let client = Client::new();

    let api_key = std::env::var("OPENWEATHER_API_KEY")?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&lang=ru&appid={}",
        city, api_key
    );

    let response = client.get(&url).send().await?;

    dbg!(&response);

    if response.status().is_success() {
        let data = dbg!(response.json::<WeatherResponse>().await)?;

        let reply = format!(
            "В городе {} петушиная погода: {}, {:.1}°C",
            data.name,
            data.weather
                .first()
                .map_or("неизвестна", |w| w.description.as_str()),
            data.main.temp
        );

        Ok(reply)
    } else {
        Ok("Я тупой пятух, нихуя не смог найти".to_string())
    }
}

#[tokio::test]
async fn test_weather() -> Result<()> {
    let text = "Эй пятушара, какая погода в городе Минск?";

    dbg!(get_weather(text).await?);

    Ok(())
}
