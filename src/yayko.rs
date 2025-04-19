use anyhow::Result;
use anyhow::bail;
use std::collections::BTreeMap;
use teloxide::Bot;
use teloxide::prelude::{Requester, UserId};
use teloxide::types::Message;
use teloxide::types::MessageEntity;
use teloxide::types::MessageKind;
use teloxide::types::{MediaKind, MessageEntityKind};
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

static USER_INFO: Mutex<BTreeMap<UserId, UserInfo>> = Mutex::const_new(BTreeMap::new());

#[derive(Debug, Clone)]
struct UserInfo {
    id: UserId,
    firstname: String,
    username: Option<String>,
    yayko_count: u64,
}

pub async fn yayko_command(bot: Bot, msg: Message) -> Result<()> {
    let id = msg.from.as_ref().unwrap().id;

    let from = msg.from.as_ref().unwrap();

    let mut user = USER_INFO.lock().await;
    let user = user.entry(id).or_insert_with(|| UserInfo {
        id,
        firstname: from.first_name.clone(),
        username: from.username.clone(),
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
            user.firstname, user.yayko_count
        ),
    )
    .await?;

    Ok(())
}

pub async fn yayko_strike(bot: Bot, msg: Message) -> Result<()> {
    let mut lock = USER_INFO.lock().await;

    let id = msg.from.as_ref().unwrap().id;
    let current_username = msg.from.as_ref().unwrap().first_name.clone();

    let from = msg.from.as_ref().unwrap();

    let mut current_user = lock
        .entry(id)
        .or_insert_with(|| UserInfo {
            id,
            firstname: from.first_name.clone(),
            username: from.username.clone(),
            yayko_count: 20,
        })
        .clone();

    let target_user = extract_user(&msg, &lock).unwrap();

    dbg!(&current_user);
    dbg!(&target_user);

    let Some(mut target_user) = target_user.clone() else {
        bot.send_message(
            msg.chat.id,
            "Этот пятух еще не зарегестрировался в игре! Пусть напишет /yayko сначала.",
        )
        .await?;

        return Ok(());
    };

    if target_user.firstname == current_username {
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
                target_user.firstname
            ),
        )
        .await?;

        return Ok(());
    }

    let mut message = String::new();

    message.push_str(&format!(
        "{} хуярит своим могучим яйцом 💪  этого пятушару: {}\n",
        current_username, target_user.firstname
    ));

    let win = rand::random::<bool>();

    if win {
        message.push_str(&format!(
            "Найййс. {} расхуярил дряхлое яйцо этого еблана {}\n",
            current_username, target_user.firstname
        ));
        target_user.yayko_count -= 1;
    } else {
        message.push_str(&format!(
                "Ахахах {} проебал далбаееебина тупая 🤣. Твое дряхлое яйцо разъебалось в щепки о великое яйцо {} 💪💪💪💪💪\n",
                current_username, target_user.firstname
            ),
        );
        current_user.yayko_count -= 1;
    }

    message.push_str(&format!(
        "У {} теперь: {}. У {}: {}\n",
        current_username, current_user.yayko_count, target_user.firstname, target_user.yayko_count
    ));

    bot.send_message(msg.chat.id, message).await?;

    lock.values_mut()
        .find(|user| dbg!(&user.firstname) == dbg!(&current_user.firstname))
        .expect(&format!(
            "User '{}' not found in USER_INFO",
            current_user.firstname
        ))
        .yayko_count = current_user.yayko_count;

    lock.values_mut()
        .find(|user| dbg!(&user.firstname) == dbg!(&target_user.firstname))
        .expect(&format!(
            "User '{}' not found in USER_INFO",
            target_user.firstname
        ))
        .yayko_count = target_user.yayko_count;

    Ok(())
}

fn extract_user(
    msg: &Message,
    lock: &MutexGuard<BTreeMap<UserId, UserInfo>>,
) -> Result<Option<UserInfo>> {
    let MessageKind::Common(ref common) = msg.kind else {
        bail!("Message is not a common message: {msg:?}");
    };

    let MediaKind::Text(ref text_media) = common.media_kind else {
        bail!("Message is not a text message: {msg:?}");
    };

    let text = &text_media.text;

    if text.contains("@") {
        let username = dbg!(extract_username2(text)).unwrap();

        let user = lock
            .values()
            .find(|user| user.username == Some(username.clone()))
            .cloned();

        return Ok(user);
    }

    let entity = text_media.entities.first().unwrap();

    let MessageEntityKind::TextMention { ref user } = entity.kind else {
        bail!("MessageEntityKind::TextMention: {msg:?}");
    };

    let target_fn = user.first_name.clone();

    let user = lock
        .values()
        .find(|user| user.firstname == target_fn)
        .cloned();

    return Ok(user);
}

fn extract_username2(text: &str) -> Option<String> {
    let at_pos = text.find('@')?;
    let rest = &text[at_pos + 1..];
    let username = rest.split_whitespace().next()?; // takes only the mention (in case of "@name extra words")
    Some(username.to_string())
}
