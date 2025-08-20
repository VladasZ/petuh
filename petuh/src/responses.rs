use std::fmt::Write;

use anyhow::Result;
use teloxide::{
    Bot,
    prelude::{ChatId, Requester},
};
use tracing::{info, instrument};

use crate::{data::DataClient, llm::petuh::SavedResponse};

const BEGIN: &str = "зул! на: ";
const END: &str = "отвечай: ";

pub fn contains_add_response_query(message: &str) -> bool {
    message.contains(BEGIN) && message.contains(END)
}

#[instrument]
pub async fn add_response(msg: &str, user: teloxide::types::User, chat_id: ChatId, bot: Bot) -> Result<()> {
    let (request, response) = parse(msg);

    let existing = DataClient::get_responses().await?;

    if existing.iter().any(|r| r.request == request) {
        bot.send_message(
            chat_id,
            format!("{}, ты шо дурны? Такой запрос уже есть", user.first_name),
        )
        .await?;
        return Ok(());
    }

    if !DataClient::add_user(&user).await?.exists {
        info!(user = format!("{user:?}"), "user added");
    }

    DataClient::add_response(SavedResponse {
        id:       0,
        user_id:  user.id.0.try_into().expect("Failed to convert user id"),
        request:  request.to_string(),
        response: response.to_string(),
    })
    .await?;

    bot.send_message(chat_id, format!("Оке, на: {request} буду отвечать: {response}"))
        .await?;

    Ok(())
}

#[instrument]
pub async fn list_responses(chat_id: ChatId, bot: Bot) -> Result<()> {
    let responses = DataClient::get_responses().await?;

    if responses.is_empty() {
        bot.send_message(chat_id, "Нэма").await?;
        return Ok(());
    }

    let mut response = "Список петушиных ответов:\n".to_string();

    for res in responses {
        let user = DataClient::get_user(res.user_id).await?;

        dbg!(&user);

        write!(
            response,
            "{} - {}. Автор: {}",
            res.request, res.response, user.first_name
        )?;

        if let Some(nickname) = user.nickname {
            write!(response, " ({})", nickname)?;
        }

        writeln!(response)?;
    }

    bot.send_message(chat_id, response).await?;

    Ok(())
}

pub async fn check_and_respond(msg: &str, chat_id: ChatId, bot: &Bot) -> Result<()> {
    let responses = DataClient::get_responses().await?;

    let Some(response) = responses.iter().find(|res| msg.to_lowercase().contains(&res.request)) else {
        return Ok(());
    };

    bot.send_message(chat_id, &response.response).await?;

    Ok(())
}

fn parse(message: &str) -> (&str, &str) {
    let query_pos = message.find(BEGIN).unwrap();
    let answer_pos = message.find(END).unwrap();

    let after_na = &message[query_pos + BEGIN.len()..answer_pos - 1];

    let after_answer = &message[answer_pos + END.len()..];

    (after_na, after_answer)
}

#[cfg(test)]
mod test {
    use crate::responses::parse;

    #[test]
    fn test() {
        assert_eq!(
            parse(&"Зул! На: сапажына в краске отвечай: крот сабака".to_lowercase()),
            ("сапажына в краске", "крот сабака")
        );
    }
}
