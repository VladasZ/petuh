#![allow(unreachable_code)]

use rand::prelude::IndexedRandom;

use crate::phrases::{NEGATIVE_ADJ, NEGATIVE_EMOJIS};
mod chat_gpt;
mod llm;
mod phrases;
mod yayko;

use anyhow::Result;
use fake::Fake;
use reqwest::Client;
use serde::Deserialize;
use teloxide::{
    ApiError, RequestError,
    prelude::*,
    sugar::request::RequestReplyExt,
    types::{InputFile, MediaKind, Message, MessageKind, ParseMode, ReactionType},
    utils::command::BotCommands,
};

use crate::{
    chat_gpt::{query_denis, query_petuh, query_zul},
    phrases::NEGATIVE,
    yayko::yayko_strike,
};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const PETUHI: &[&str] = &["Максим", "Владик", "Владас", "Рома", "Настя", "Алёна", "Витёк"];

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = r"🐓 Петушиные команды:")]
enum Command {
    #[command(description = "Помощь петушары.")]
    Help,
    #[command(description = "Приветствие.")]
    K,
    #[command(description = "Уважение Алене. 🐓")]
    A,
    #[command(description = "Уважение Максиму. 🐓")]
    M,
    #[command(description = "Уважение Роме. 🐓")]
    R,
    #[command(description = "Уважение Владику. 🐓🐓")]
    V,
    #[command(description = "Уважение Владасу.")]
    VS,
    #[command(description = "Уважение Насте.")]
    N,
    #[command(description = "Уважение Денису. 🐓🐓🐓🐓🐓🐓🐓")]
    D,
    #[command(description = "Доброе утро.")]
    Gm,
    #[command(description = "Узнать кто петух.")]
    Kto,
    #[command(description = "Бросить петушиный кубик.")]
    Kub,
    #[command(description = "Разъебать.")]
    Rz,
    Vladik,
}

const O4KO_STRENGTH: u32 = 40;
const COMMENT_PROBABILITY: u32 = 80;
const SHUT_UP_PROBABILITY: u32 = 110;
const REACTION_PROBABILITY: u32 = 30;

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    dbg!(&msg);

    if (0..15).fake::<u32>() == 5 {
        bot.send_animation(
            msg.chat.id,
            InputFile::file_id(
                "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE".into(),
            ),
        )
        .reply_to(msg.id)
        .await?;
        return Ok(());
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::K => {
            bot.send_message(
                msg.chat.id,
                "Кукарекуууу я петушара!!! Я кукорекою як пятух 🐓. Кок",
            )
            .await?;
        }
        Command::M => {
            bot.send_message(msg.chat.id, "Максим Пятушара!!!! 🐓🐓🐓🐓🐓").await?;
        }
        Command::R => {
            bot.send_message(msg.chat.id, "🐓Рома🐓 каблук петушиный 👠").await?;
        }
        Command::Gm => {
            let morning = query_zul(
                "Зул, напиши очень доброе поздравление с добрым утром, с теплом и позитивом, пожелай всем \
                 участникам чата хорошего дня. Добавь эмодзи разных цветов сердечек и петухов. Но все равно \
                 под конец скажи что нибудь немного оскорбительное.",
            )
            .await
            .expect("Failed to gm");

            bot.send_message(msg.chat.id, morning).await?;
        }
        Command::Kto => {
            let name = PETUHI.choose(&mut rand::rng()).unwrap();
            let neg = NEGATIVE.choose(&mut rand::rng()).unwrap();
            let neg_emoji = NEGATIVE_EMOJIS.choose(&mut rand::rng()).unwrap();
            let neg_adj = NEGATIVE_ADJ.choose(&mut rand::rng()).unwrap();
            let reply = format!("{name} {neg} — {neg_adj} пятух! 🐓 {neg_emoji}");
            bot.send_message(msg.chat.id, reply).await?;
        }
        Command::Kub => {
            bot.send_dice(msg.chat.id).await?;
        }
        Command::Rz => {
            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE".into(),
                ),
            )
            .reply_to(msg.id)
            .await?;
        }
        Command::Vladik => {
            let user_id = 1_302_643_454;
            let message = format!("Это великий пятушара - <a href=\"tg://user?id={user_id}\">Пятух!!!</a>!",);

            bot.send_message(msg.chat.id, message).parse_mode(ParseMode::Html).await?;
        }
        Command::V => {
            bot.send_message(msg.chat.id, "@blvcklawyer 🐓!! Ты че ахуел??!!!").await?;

            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAyEFAASIlB1pAAEBYNFn_iIqy0BjM-b3xUwvtxoYkpDWgQACcGAAAmh_cUkNpnr54Lr50TYE".into(),
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeSBoHgSeyVZW8QWT4g-O5z4urDL1QwACqXIAAj-O8EidWMzkpLfNWDYE".into(),
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeR5oHgP87elHbOfzEmHL6OS6Ehs6NwAC-moAAnfz8EjrSFgCXzoEujYE".into(),
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeR1oHgNpLi9JjdvGyQYi58R1K5SKowAC9GoAAnfz8EgtCn1BYGHvhTYE".into(),
                ),
            )
            .await?;
        }
        Command::D => {
            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAx0CctKdnAACsWBoGLtHnaEy0-Qy8rC0lMUmA520CwACUWMAAhH3wUqv_Bq7iSTS3jYE".into(),
                ),
            )
            .await?;

            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAyEFAASIlB1pAAEBYNFn_iIqy0BjM-b3xUwvtxoYkpDWgQACcGAAAmh_cUkNpnr54Lr50TYE".into(),
                ),
            )
            .await?;

            bot.send_message(msg.chat.id, "в стойло подзетник").await?;
            bot.send_message(msg.chat.id, "только и можешь что сракой их сперму ловить")
                .await?;
        }
        Command::VS => {
            bot.send_message(msg.chat.id, "Владас петух сцаный 🐓").await?;
        }
        Command::N => {
            bot.send_message(msg.chat.id, "Настя пятух 🐓").await?;
        }
        Command::A => {
            bot.send_message(msg.chat.id, "Алена пятух !!! 🐓🐓🐓🐓🐓🐓🐓🐓🐓🐓🐓").await?;
            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAxkBAAIXDWhJOHImiHv4j1Q5TorZFGdfqUw7AAJ7ggACgXBJSgM0dD0qhhVZNgQ".into(),
                ),
            )
            .await?;
            bot.send_message(
                msg.chat.id,
                "Слушай, Алена 🐓. Пора тебе валить обратно в свой курятник. Там твои петушары ждут. Чего \
                 ты тут мотаешься? Занимайся своими делами.",
            )
            .await?;
        }
    }
    Ok(())
}

async fn handle_text(bot: Bot, msg: Message) -> ResponseResult<()> {
    const REACTIONS: &[&str] = &["🤡", "🔥", "💯", "💊", "🤮", "🤩", "👏", "💩"];

    dbg!(&msg);

    if (0..REACTION_PROBABILITY).fake::<u32>() == 0 {
        let mut reaction = bot.set_message_reaction(msg.chat.id, msg.id);

        let emoji = REACTIONS.choose(&mut rand::rng()).unwrap();

        reaction.reaction = Some(vec![ReactionType::Emoji {
            emoji: (*emoji).to_string(),
        }]);

        reaction.send().await?;
    }

    if (0..SHUT_UP_PROBABILITY).fake::<u32>() == 5 {
        bot.send_sticker(
            msg.chat.id,
            InputFile::file_id(
                "CAACAgIAAxkBAAIRX2g4zKi0qtqmsZX-QPKaN-p0czM2AAJZeAACbdzISWfvVJ7Ij4tfNgQ".into(),
            ),
        )
        .reply_to(msg.id)
        .await?;

        return Ok(());
    }

    if (0..COMMENT_PROBABILITY).fake::<u32>() == 5 {
        bot.send_animation(
            msg.chat.id,
            InputFile::file_id(
                "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE".into(),
            ),
        )
        .reply_to(msg.id)
        .await?;
        return Ok(());
    }

    match msg.kind {
        MessageKind::Common(ref common_message) => match &common_message.media_kind {
            MediaKind::Sticker(sticker) => {
                if sticker.sticker.file.unique_id == "AgADl14AAqISEEs".into() {
                    bot.send_message(msg.chat.id, "@maxon8871 !! Ну ты и петух!!! 🐓🐓").await?;
                }
            }
            _ => (),
        },
        _ => (),
    }

    if let Some(text) = msg.text() {
        let text = text.to_lowercase();

        if text.contains("я тупой пятух") {
            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgQAAyEFAASIlB1pAAEBWKZn9kmLfI2kj6gd4nMKqouqoDMW1gACowIAAij8FFPkdVtUyi5cBTYE".into(),
                ),
            )
            .reply_to(msg.id)
            .await?;

            return Ok(());
        }

        if text.starts_with("хуярю яйцом") || text.starts_with("Хуярю яйцом") {
            let result = yayko_strike(bot.clone(), msg.clone()).await.map_err(|e| {
                dbg!(&e);
                RequestError::Api(ApiError::CantParseUrl)
            });

            if let Err(err) = result {
                bot.send_message(msg.chat.id, format!("Я обосрался: {err:?}")).await?;
            }

            return Ok(());
        }

        if text.starts_with("денис, ") {
            let text = &text["денис, ".len()..];

            bot.send_message(
                msg.chat.id,
                format!("Денис:\n{}", query_denis(text).await.unwrap()),
            )
            .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    query_denis(
                        "напиши сообщение как будто у тебя сгорела жопа и ты уходишь из чата и плевал на \
                         всех его участников",
                    )
                    .await
                    .unwrap(),
                )
                .await?;
                bot.leave_chat(msg.chat.id).await?;
            }

            return Ok(());
        }

        if text.starts_with("пятух, ") {
            let text = &text["пятух, ".len()..];

            bot.send_message(
                msg.chat.id,
                format!("Пятух:\n{}", query_petuh(text).await.unwrap()),
            )
            .await?;

            return Ok(());
        }

        if text.starts_with("зул, ") {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Зул:\n{}",
                    query_zul(text.strip_prefix("зул, ").expect("Failed to strip zyl"))
                        .await
                        .unwrap()
                ),
            )
            .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    query_zul(
                        "напиши сообщение как будто у тебя сгорела жопа и ты уходишь из чата и плевал на \
                         всех его участников",
                    )
                    .await
                    .unwrap(),
                )
                .await?;
                bot.leave_chat(msg.chat.id).await?;
            }

            return Ok(());
        }

        if text.contains("--version") || text.contains("-v") {
            bot.send_message(
                msg.chat.id,
                format!(
                    r"
Курятник v{APP_VERSION}

Вероятность комментария: {COMMENT_PROBABILITY}
Вероятность реакции: {REACTION_PROBABILITY}

Доступные интерактивные петухи для общения:
- Денис: Стойкость очка: {O4KO_STRENGTH}
- Зул: Стойкость очка: {O4KO_STRENGTH}
- Пятух: Стойкость очка: не сгорает

Список петухов:
{}

(Владик Пятушара Ванючы)
",
                    PETUHI.join("\n"),
                ),
            )
            .await?;
        }

        if text.contains("погода") {
            let weather = get_weather(&text).await.unwrap();
            bot.send_message(msg.chat.id, weather).await?;
        }

        if text == "Кто петух?" {
            let name = PETUHI.choose(&mut rand::rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");

    pretty_env_logger::init();

    dotenv::dotenv().ok();

    let _guard = sentry::init((
        std::env::var("SENTRY_LINK")?,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    log::info!("Starting Telegram bot...");

    let bot = Bot::from_env();

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
    log::info!("Starting Telegram bot...");

    let bot = Bot::from_env();

    let chat = bot.get_chat(ChatId(1)).await?;

    dbg!(&chat);

    bot.delete_message(ChatId(1), teloxide::types::MessageId(1)).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main:    Main,
    name:    String,
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
    let city = city.split(' ').next_back().unwrap();

    dbg!(&city);

    dotenv::dotenv()?;

    let client = Client::new();

    let api_key = std::env::var("OPENWEATHER_API_KEY")?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&lang=ru&appid={api_key}",
    );

    let response = client.get(&url).send().await?;

    dbg!(&response);

    if response.status().is_success() {
        let data = dbg!(response.json::<WeatherResponse>().await)?;

        let reply = format!(
            "В городе {} петушиная погода: {}, {:.1}°C",
            data.name,
            data.weather.first().map_or("неизвестна", |w| w.description.as_str()),
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

#[test]
fn system_info() {
    dbg!(APP_VERSION);
}
