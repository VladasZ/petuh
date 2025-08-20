use fake::Fake;
use rand::prelude::IndexedRandom;
use teloxide::{
    ApiError, Bot, RequestError,
    prelude::{Request, Requester},
    requests::ResponseResult,
    sugar::request::RequestReplyExt,
    types::{InputFile, MediaKind, Message, MessageKind, ReactionType},
};
use tracing::instrument;

use crate::{
    APP_VERSION, PETUHI,
    llm::{LLMClient, petuh::Personality},
    responses::{add_response, check_and_respond, contains_add_response_query},
    weather::get_weather,
    yayko::yayko_strike,
};

const O4KO_STRENGTH: u32 = 40;
const COMMENT_PROBABILITY: u32 = 80;
const SHUT_UP_PROBABILITY: u32 = 110;
const REACTION_PROBABILITY: u32 = 30;

#[instrument]
pub async fn handle_text(bot: Bot, msg: Message) -> ResponseResult<()> {
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

    check_and_respond(msg.text().unwrap_or_default(), msg.chat.id, &bot)
        .await
        .map_err(|err| RequestError::Api(ApiError::Unknown(err.to_string())))?;

    if let MessageKind::Common(ref common_message) = msg.kind
        && let MediaKind::Sticker(sticker) = &common_message.media_kind
        && sticker.sticker.file.unique_id == "AgADl14AAqISEEs".into()
    {
        bot.send_message(msg.chat.id, "@maxon8871 !! Ну ты и петух!!! 🐓🐓").await?;
    }

    if let Some(text) = msg.text() {
        let text = text.to_lowercase();

        if contains_add_response_query(&text) {
            add_response(&text, msg.chat.id, bot)
                .await
                .map_err(|err| RequestError::Api(ApiError::Unknown(err.to_string())))?;
            return Ok(());
        }

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

        if text.starts_with("хуярю яйцом") {
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
            let text = text.strip_prefix("денис, ").unwrap();

            bot.send_message(
                msg.chat.id,
                format!(
                    "Денис:\n{}",
                    LLMClient::request(Personality::Denis, text).await.unwrap()
                ),
            )
            .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    LLMClient::request(
                        Personality::Denis,
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
            let text = text.strip_prefix("пятух, ").unwrap();

            bot.send_message(
                msg.chat.id,
                format!(
                    "Пятух:\n{}",
                    LLMClient::request(Personality::Petuh, text).await.unwrap()
                ),
            )
            .await?;

            return Ok(());
        }

        if text.starts_with("зул, ") {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Зул:\n{}",
                    LLMClient::request(
                        Personality::Zyl,
                        text.strip_prefix("зул, ").expect("Failed to strip zyl")
                    )
                    .await
                    .unwrap()
                ),
            )
            .await?;

            if (0..O4KO_STRENGTH).fake::<u32>() == 5 {
                bot.send_message(
                    msg.chat.id,
                    LLMClient::request(
                        Personality::Zyl,
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
