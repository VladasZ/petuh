use anyhow::Result;
use anyhow::bail;
use std::collections::BTreeMap;
use teloxide::Bot;
use teloxide::prelude::{Requester, UserId};
use teloxide::types::MediaKind;
use teloxide::types::Message;
use teloxide::types::MessageKind;
use tokio::sync::Mutex;

static USER_INFO: Mutex<BTreeMap<UserId, UserInfo>> = Mutex::const_new(BTreeMap::new());

#[derive(Debug, Clone)]
struct UserInfo {
    id: UserId,
    username: String,
    yayko_count: u64,
}

pub async fn yayko_command(bot: Bot, msg: Message) -> Result<()> {
    let id = msg.from.as_ref().unwrap().id;
    let username = msg
        .from
        .as_ref()
        .unwrap()
        .username
        .clone()
        .unwrap_or_else(|| msg.from.as_ref().unwrap().first_name.clone());

    let mut user = USER_INFO.lock().await;
    let user = user.entry(id).or_insert_with(|| UserInfo {
        id,
        username: username.to_string(),
        yayko_count: 20,
    });

    if user.yayko_count == 0 {
        bot.send_message(msg.chat.id, "Ты уже проебал, у тебя нихуя нету, отъебись.")
            .await?;

        return Ok(());
    }

    bot.send_message(
        msg.chat.id,
        format!(
            "{}, хочешь узнать сколько у тебя яиц? У тебя: {}",
            user.username, user.yayko_count
        ),
    )
    .await?;

    Ok(())
}

pub async fn yayko_strike(bot: Bot, msg: Message) -> Result<()> {
    let mut lock = USER_INFO.lock().await;

    let id = msg.from.as_ref().unwrap().id;
    let current_username = msg
        .from
        .as_ref()
        .unwrap()
        .username
        .clone()
        .unwrap_or_else(|| msg.from.as_ref().unwrap().first_name.clone());

    let current_user = lock
        .values()
        .find(|user| user.username == current_username.clone());

    let Some(mut current_user) = current_user.cloned() else {
        bot.send_message(
            msg.chat.id,
            format!(
                "{} Ну ты и Пятушара!! Еще не зарегестрировался в игре! Сначала напиши /yayko",
                current_username
            ),
        )
        .await?;

        return Ok(());
    };

    let MessageKind::Common(ref common) = msg.kind else {
        bail!("Message is not a common message: {msg:?}");
    };

    let MediaKind::Text(ref text) = common.media_kind else {
        bail!("Message is not a text message: {msg:?}");
    };

    let text = text.text.clone();

    let target_username = extract_username(&text).unwrap();

    let target_user = lock
        .values()
        .find(|user| user.username == target_username.clone())
        .cloned();

    let Some(mut target_user) = target_user.clone() else {
        bot.send_message(
            msg.chat.id,
            format!(
                "{} Пятушара!! Еще не зарегестрировался в игре! Пусть напишет /yayko сначала.",
                target_username
            ),
        )
        .await?;

        return Ok(());
    };

    if target_user.username == current_username {
        bot.send_message(msg.chat.id, "Хочешь сам себя уебать? Ты шо еблан?")
            .await?;

        return Ok(());
    }

    if current_user.yayko_count == 0 {
        bot.send_message(
            msg.chat.id,
            "Aахаха так ты пятух уже проебал все яйца! Пшол нахуй!!1",
        )
        .await?;

        return Ok(());
    }

    if target_user.yayko_count == 0 {
        bot.send_message(
            msg.chat.id,
            format!(
                "Этот пятух {} уже проебал все свои яйца!! Хуле ты доебался?",
                target_username
            ),
        )
        .await?;

        return Ok(());
    }

    let mut message = String::new();

    message.push_str(&format!(
        "{} хуярит своим могучим яйцом 💪  этого пятушару: {}\n",
        current_username, target_username
    ));

    let win = rand::random::<bool>();

    if win {
        message.push_str(&format!(
            "Найййс. {} расхуярил дряхлое яйцо этого еблана {}\n",
            current_username, target_username
        ));
        target_user.yayko_count -= 1;
    } else {
        message.push_str(&format!(
                "Ахахах {} проебал далбаееебина тупая 🤣. Твое дряхлое яйцо разъебалось в щепки о великое яйцо {} 💪💪💪💪💪\n",
                current_username, target_username
            ),
        );
        current_user.yayko_count -= 1;
    }

    message.push_str(&format!(
        "У {} теперь: {}. У {}: {}\n",
        current_username, current_user.yayko_count, target_username, target_user.yayko_count
    ));

    bot.send_message(msg.chat.id, message).await?;

    lock.values_mut()
        .find(|user| dbg!(&user.username) == dbg!(&current_user.username))
        .expect(&format!(
            "User '{}' not found in USER_INFO",
            current_user.username
        ))
        .yayko_count = current_user.yayko_count;

    lock.values_mut()
        .find(|user| dbg!(&user.username) == dbg!(&target_user.username))
        .expect(&format!(
            "User '{}' not found in USER_INFO",
            target_user.username
        ))
        .yayko_count = target_user.yayko_count;

    Ok(())
}

fn extract_username(text: &str) -> Option<String> {
    let at_pos = text.find('@')?;
    let rest = &text[at_pos + 1..];
    let username = rest.split_whitespace().next()?;
    Some(username.to_string())
}
