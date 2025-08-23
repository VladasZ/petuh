use anyhow::{Context, Result};
use chrono::Local;
use common::Redis;
use teloxide::types::ChatId;

pub trait UserExtension {
    fn set_last_message_timepestamp(&self, chat_id: &ChatId) -> Result<()>;
}

impl UserExtension for teloxide::types::User {
    fn set_last_message_timepestamp(&self, chat_id: &ChatId) -> Result<()> {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        Redis::set(
            format!(
                "user_last_message:chat:<{}>:user:<{}>:user_name:<{}>",
                chat_id, self.id.0, self.first_name
            ),
            timestamp,
        )
        .context("Failed to set_last_message_timepestamp")?;

        Ok(())
    }
}
