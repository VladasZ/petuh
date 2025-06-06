#![allow(unreachable_code)]

mod chat_gpt;
mod phrases;
mod yayko;

use anyhow::Result;
use fake::Fake;
use rand::prelude::SliceRandom;
use reqwest::Client;
use serde::Deserialize;
use teloxide::{
    ApiError, RequestError,
    prelude::*,
    sugar::request::RequestReplyExt,
    types::{InputFile, MediaKind, Message, MessageKind, ParseMode, ReactionType},
    utils::command::BotCommands,
};
use whoami::fallible;

use crate::{
    chat_gpt::{query_denis, query_petuh, query_zul},
    yayko::yayko_strike,
};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const PETUHI: &[&str] = &["Максим", "Владик", "Владас", "Рома", "Настя", "Денис"];

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = r"🐓 Петушиные команды:")]
enum Command {
    #[command(description = "Помощь петушары.")]
    Help,
    #[command(description = "Приветствие.")]
    K,
    #[command(description = "Уважение Максиму.")]
    M,
    #[command(description = "Уважение Роме.")]
    R,
    #[command(description = "Уважение Владику.")]
    V,
    #[command(description = "Уважение Владасу.")]
    VS,
    #[command(description = "Уважение Денису.")]
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

const O4KO_STRENGTH: u32 = 28;
const COMMENT_PROBABILITY: u32 = 50;
const SHUT_UP_PROBABILITY: u32 = 80;
const REACTION_PROBABILITY: u32 = 20;

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    dbg!(&msg);

    if (0..15).fake::<u32>() == 5 {
        bot.send_animation(
            msg.chat.id,
            InputFile::file_id(
                "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE",
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
            bot.send_message(msg.chat.id, "Рома каблук петушиный 👠").await?;
        }
        Command::Gm => {
            bot.send_message(msg.chat.id, "Доброе утро петушары ебаные! 🐓").await?;
        }
        Command::Kto => {
            let name = PETUHI.choose(&mut rand::thread_rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
        Command::Kub => {
            bot.send_dice(msg.chat.id).await?;
        }
        Command::Rz => {
            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE",
                ),
            )
            .reply_to(msg.id)
            .await?;
        }
        Command::Vladik => {
            let user_id = 1302643454; // Replace with actual user ID
            let message = format!(
                "Это великий пятушара - <a href=\"tg://user?id={}\">Пятух!!!</a>!",
                user_id
            );

            bot.send_message(msg.chat.id, message).parse_mode(ParseMode::Html).await?;
        }
        Command::V => {
            bot.send_message(msg.chat.id, "@blvcklawyer !! Ты че ахуел??!!!").await?;

            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAyEFAASIlB1pAAEBYNFn_iIqy0BjM-b3xUwvtxoYkpDWgQACcGAAAmh_cUkNpnr54Lr50TYE",
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeSBoHgSeyVZW8QWT4g-O5z4urDL1QwACqXIAAj-O8EidWMzkpLfNWDYE",
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeR5oHgP87elHbOfzEmHL6OS6Ehs6NwAC-moAAnfz8EjrSFgCXzoEujYE",
                ),
            )
            .await?;

            bot.send_animation(
                msg.chat.id,
                InputFile::file_id(
                    "CgACAgIAAyEFAASIlB1pAAEBeR1oHgNpLi9JjdvGyQYi58R1K5SKowAC9GoAAnfz8EgtCn1BYGHvhTYE",
                ),
            )
            .await?;
        }
        Command::D => {
            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAx0CctKdnAACsWBoGLtHnaEy0-Qy8rC0lMUmA520CwACUWMAAhH3wUqv_Bq7iSTS3jYE",
                ),
            )
            .await?;

            bot.send_sticker(
                msg.chat.id,
                InputFile::file_id(
                    "CAACAgIAAyEFAASIlB1pAAEBYNFn_iIqy0BjM-b3xUwvtxoYkpDWgQACcGAAAmh_cUkNpnr54Lr50TYE",
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
    }
    Ok(())
}

async fn handle_text(bot: Bot, msg: Message) -> ResponseResult<()> {
    dbg!(&msg);

    if (0..REACTION_PROBABILITY).fake::<u32>() == 0 {
        let mut reaction = bot.set_message_reaction(msg.chat.id, msg.id);

        const REACTIONS: &[&str] = &["🤡", "🔥", "💯"];

        // "👍", "👎", "❤", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉",
        // "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚",
        // "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕",
        // "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍",
        // "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘",
        // "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"

        let emoji = REACTIONS.choose(&mut rand::thread_rng()).unwrap();

        reaction.reaction = Some(vec![ReactionType::Emoji {
            emoji: emoji.to_string(),
        }]);

        reaction.send().await?;
    }

    if (0..SHUT_UP_PROBABILITY).fake::<u32>() == 5 {
        bot.send_sticker(
            msg.chat.id,
            InputFile::file_id("CAACAgIAAxkBAAIRX2g4zKi0qtqmsZX-QPKaN-p0czM2AAJZeAACbdzISWfvVJ7Ij4tfNgQ"),
        )
        .reply_to(msg.id)
        .await?;

        return Ok(());
    }

    if (0..COMMENT_PROBABILITY).fake::<u32>() == 5 {
        bot.send_animation(
            msg.chat.id,
            InputFile::file_id(
                "CgACAgIAAyEFAASIlB1pAAEBW3Jn95C0FYLjR1ttXMGad8DtIkPSIQACSVgAAtq2yUpGoSZCA0YzmjYE",
            ),
        )
        .reply_to(msg.id)
        .await?;
        return Ok(());
    }

    match msg.kind {
        MessageKind::Common(ref common_message) => match &common_message.media_kind {
            MediaKind::Sticker(sticker) => {
                if sticker.sticker.file.unique_id == "AgADl14AAqISEEs" {
                    bot.send_message(msg.chat.id, "@maxon8871 !! Ну ты и петух!!!").await?;
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
                    "CgACAgQAAyEFAASIlB1pAAEBWKZn9kmLfI2kj6gd4nMKqouqoDMW1gACowIAAij8FFPkdVtUyi5cBTYE",
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
                bot.send_message(msg.chat.id, format!("Я обосрался: {:?}", err)).await?;
            }

            return Ok(());
        }

        if text.starts_with("денис, ") {
            let text = &text["денис, ".len()..];

            bot.send_message(
                msg.chat.id,
                format!("Денис:\n{}", query_denis(&text).await.unwrap()),
            )
            .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    query_denis(
                        &"напиши сообщение как будто у тебя сгорела жопа и ты уходишь из чата и плевал на \
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
                format!("Пятух:\n{}", query_petuh(&text).await.unwrap()),
            )
            .await?;

            return Ok(());
        }

        if text.starts_with("зул, ") {
            let text = &text["зул, ".len()..];

            bot.send_message(msg.chat.id, format!("Зул:\n{}", query_zul(&text).await.unwrap()))
                .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    query_zul(
                        &"напиши сообщение как будто у тебя сгорела жопа и ты уходишь из чата и плевал на \
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

Кручусь тут:
{}

(Владик Пятушара Ванючы)
",
                    PETUHI.join("\n"),
                    collect_system_info()
                ),
            )
            .await?;
        }

        if text.contains("погода") {
            let weather = get_weather(&text).await.unwrap();
            bot.send_message(msg.chat.id, weather).await?;
        }

        if text == "Кто петух?" {
            let name = PETUHI.choose(&mut rand::thread_rng()).unwrap();
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

    bot.delete_message(ChatId(1), MessageId(1)).await?;

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

fn collect_system_info() -> String {
    let mut info = String::new();

    // if let Ok(os_type) = sys_info::os_type() {
    //     info += &format!("OS Type: {}\n", os_type);
    // }
    //
    // if let Ok(os_release) = sys_info::os_release() {
    //     info += &format!("OS Release: {}\n", os_release);
    // }

    let hostname = fallible::hostname().unwrap();
    info += &format!("Hostname: {}\n", hostname);

    if let Ok(uname) = uname::uname() {
        info += &format!("Architecture: {}\n", uname.machine);
        info += &format!("Sysname: {}\n", uname.sysname);
        info += &format!("Nodename: {}\n", uname.nodename);
        info += &format!("Kernel Release: {}\n", uname.release);
        info += &format!("Version: {}\n", uname.version);
    }

    info += &format!("Distro: {}\n", whoami::distro());
    info += &format!("Username: {}\n", whoami::username());
    info += &format!("Desktop Environment: {}\n", whoami::desktop_env());
    info += &format!("Platform: {}\n", whoami::platform());

    info
}

#[test]
fn system_info() {
    println!("{}", &collect_system_info());
}
