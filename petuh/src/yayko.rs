use std::collections::BTreeMap;

use anyhow::{Result, anyhow, bail};
use rand::prelude::IndexedRandom;
use teloxide::{
    Bot,
    prelude::{Requester, UserId},
    types::{ChatId, MediaKind, Message, MessageEntityKind, MessageKind},
};
use tokio::sync::Mutex;

use crate::phrases::{NEGATIVE, NEGATIVE_EMOJIS, POSITIVE, POSITIVE_EMOJIS, negative_word};

const STARTING_YAYKO: u64 = 25;

static USER_INFO: Mutex<BTreeMap<ChatId, BTreeMap<UserId, UserInfo>>> = Mutex::const_new(BTreeMap::new());

#[derive(Debug, Clone)]
struct UserInfo {
    firstname:   String,
    username:    Option<String>,
    yayko_count: u64,
}

pub async fn _yayko_stats(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        format!("Ты шо {}? Пасха кончилась.", negative_word()?),
    )
    .await?;

    return Ok(());

    let mut chats = USER_INFO.lock().await;

    let chat = chats.entry(msg.chat.id).or_default();

    if chat.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Ни у кого нет яиц, вы все пятухи! Сначала отправьте /yayko в чат!",
        )
        .await?;

        return Ok(());
    }

    let mut result = "Статистика курятника:\n".to_string();

    for user in chat.values() {
        if user.yayko_count == 0 {
            result.push_str(&format!(
                "Пхахахха {} {} {} проебал уже все яйца!! Чтобы восстановить яйца напиши: я тупой пятух\n",
                user.firstname,
                NEGATIVE.choose(&mut rand::rng()).ok_or(anyhow!("random"))?,
                NEGATIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?
            ));
        } else {
            result.push_str(&format!("{} у тебя: {}\n", user.firstname, user.yayko_count));
        }
    }

    bot.send_message(msg.chat.id, result).await?;

    Ok(())
}

pub async fn _yayko_command(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        format!("Ты шо {}? Пасха кончилась.", negative_word()?),
    )
    .await?;

    return Ok(());

    let id = msg.from.as_ref().ok_or(anyhow!("msg.from.as_ref()"))?.id;

    let from = msg.from.as_ref().ok_or(anyhow!("msg.from.as_ref()"))?;

    let mut chats = USER_INFO.lock().await;

    let chat = chats.entry(msg.chat.id).or_default();

    let user = chat.entry(id).or_insert_with(|| UserInfo {
        firstname:   from.first_name.clone(),
        username:    from.username.clone(),
        yayko_count: STARTING_YAYKO,
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
    bot.send_message(
        msg.chat.id,
        format!("Ты шо {}? Пасха кончилась.", negative_word()?),
    )
    .await?;

    return Ok(());

    let mut chats = USER_INFO.lock().await;

    let chat = chats.entry(msg.chat.id).or_default();

    let id = msg.from.as_ref().ok_or(anyhow!("No from"))?.id;
    let current_username = msg.from.as_ref().ok_or(anyhow!("No current username"))?.first_name.clone();

    let from = msg.from.as_ref().ok_or(anyhow!("No from"))?;

    let mut current_user = chat
        .entry(id)
        .or_insert_with(|| UserInfo {
            firstname:   from.first_name.clone(),
            username:    from.username.clone(),
            yayko_count: STARTING_YAYKO,
        })
        .clone();

    let target_user = extract_user(&msg, chat)?;

    dbg!(&current_user);
    dbg!(&target_user);

    let Some(mut target_user) = target_user.clone() else {
        bot.send_message(
            msg.chat.id,
            format!(
                "Этот {} еще не зарегестрировался в игре {}!  Пусть напишет /yayko сначала.",
                NEGATIVE.choose(&mut rand::rng()).ok_or(anyhow!("random"))?,
                NEGATIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?
            ),
        )
        .await?;

        return Ok(());
    };

    if target_user.firstname == current_username {
        bot.send_message(msg.chat.id, "Хочешь сам себя уебать? Ты шо еблан?").await?;

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

    let positive = POSITIVE.choose(&mut rand::rng()).ok_or(anyhow!("random"))?;
    let negative = NEGATIVE.choose(&mut rand::rng()).ok_or(anyhow!("random"))?;

    let pos_emoji = format!(
        "{}{}",
        POSITIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?,
        POSITIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?
    );

    let neg_emoji = format!(
        "{}{}",
        NEGATIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?,
        NEGATIVE_EMOJIS.choose(&mut rand::rng()).ok_or(anyhow!("random"))?
    );

    if win {
        message.push_str(&format!(
            "Найййс {pos_emoji}. {positive} {current_username} расхуярил дряхлое яйцо этого еблана {}! {} \
             да ты {negative} {neg_emoji}!\n",
            target_user.firstname, target_user.firstname
        ));

        target_user.yayko_count -= 1;
    } else {
        message.push_str(&format!(
            "Ахахах {current_username} проебал как {negative} {neg_emoji}. Твое дряхлое яйцо разъебалось в \
             щепки о великое яйцо {} 💪💪💪💪💪\n {} {positive} {pos_emoji}\n",
            target_user.firstname, target_user.firstname
        ));
        current_user.yayko_count -= 1;
    }

    message.push_str(&format!(
        "У {} теперь: {}. У {}: {}\n",
        current_username, current_user.yayko_count, target_user.firstname, target_user.yayko_count
    ));

    bot.send_message(msg.chat.id, message).await?;

    chat.values_mut()
        .find(|user| dbg!(&user.firstname) == dbg!(&current_user.firstname))
        .unwrap_or_else(|| panic!("User '{}' not found in USER_INFO", current_user.firstname))
        .yayko_count = current_user.yayko_count;

    chat.values_mut()
        .find(|user| dbg!(&user.firstname) == dbg!(&target_user.firstname))
        .unwrap_or_else(|| panic!("User '{}' not found in USER_INFO", target_user.firstname))
        .yayko_count = target_user.yayko_count;

    Ok(())
}

fn extract_user(msg: &Message, users: &BTreeMap<UserId, UserInfo>) -> Result<Option<UserInfo>> {
    let MessageKind::Common(ref common) = msg.kind else {
        bail!("Message is not a common message: {msg:?}");
    };

    let MediaKind::Text(ref text_media) = common.media_kind else {
        bail!("Message is not a text message: {msg:?}");
    };

    let text = &text_media.text;

    if text.contains('@') {
        let username = dbg!(extract_username2(text)).ok_or(anyhow!("extract_username2"))?;

        let user = users.values().find(|user| user.username == Some(username.clone())).cloned();

        return Ok(user);
    }

    let entity = text_media.entities.first().ok_or(anyhow!("text_media.entities.first()"))?;

    let MessageEntityKind::TextMention { ref user } = entity.kind else {
        bail!("MessageEntityKind::TextMention: {msg:?}");
    };

    let target_fn = user.first_name.clone();

    let user = users.values().find(|user| user.firstname == target_fn).cloned();

    Ok(user)
}

fn extract_username2(text: &str) -> Option<String> {
    let at_pos = text.find('@')?;
    let rest = &text[at_pos + 1..];
    let username = rest.split_whitespace().next()?; // takes only the mention (in case of "@name extra words")
    Some(username.to_string())
}
