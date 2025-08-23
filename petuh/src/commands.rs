use fake::Fake;
use teloxide::{
    ApiError, Bot, RequestError,
    payloads::SendMessageSetters,
    prelude::{Message, Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::{InputFile, ParseMode},
    utils::command::BotCommands,
};
use tracing::instrument;

use crate::{
    llm::{LLMClient, petuh::Personality},
    phrases::{kto, vladik_jopoliz},
    responses::list_responses,
    user_extension::UserExtension,
};

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase", description = r"🐓 Петушиные команды:")]
pub enum Command {
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
    #[command(description = "Уважение Владику 2. 🐓🐓🐓🐓")]
    J,
    #[command(description = "Уважение Владасу.")]
    VS,
    #[command(description = "Уважение Насте.")]
    N,
    #[command(description = "Доброе утро.")]
    Gm,
    #[command(description = "Узнать кто петух.")]
    Kto,
    #[command(description = "Бросить петушиный кубик.")]
    Kub,
    #[command(description = "Разъебать.")]
    Rz,
    #[command(description = "Список ответов.")]
    Otvet,
    Vladik,
}

#[instrument]
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    dbg!(&msg);

    if let Some(ref user) = msg.from {
        user.set_last_message_timepestamp(&msg.chat.id)
            .map_err(|err| RequestError::Api(ApiError::Unknown(err.to_string())))?;
    }

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
            let morning = LLMClient::request(
                Personality::Zyl,
                "Зул, напиши очень доброе поздравление с добрым утром, с теплом и позитивом, пожелай всем \
                 участникам чата хорошего дня. Добавь эмодзи разных цветов сердечек и петухов. Но все равно \
                 под конец скажи что нибудь оскорбительное.",
            )
            .await
            .expect("Failed to gm");

            bot.send_message(msg.chat.id, morning).await?;
        }
        Command::Kto => {
            bot.send_message(msg.chat.id, kto()).await?;
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
        Command::J => {
            bot.send_message(msg.chat.id, vladik_jopoliz()).await?;
        }
        Command::Otvet => {
            list_responses(msg.chat.id, bot)
                .await
                .map_err(|err| RequestError::Api(ApiError::Unknown(err.to_string())))?;
        }
    }
    Ok(())
}
