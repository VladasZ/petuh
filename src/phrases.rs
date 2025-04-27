use anyhow::Result;
use anyhow::anyhow;
use rand::prelude::SliceRandom;

pub const POSITIVE: &[&str] = &[
    "богачь",
    "кароль",
    "великий",
    "царь",
    "титан",
    "хозяин жизни",
    "барин",
    "котик",
    "герой",
    "имба",
];

pub const NEGATIVE: &[&str] = &[
    "простофиля",
    "олух",
    "дурнина",
    "лах",
    "доун",
    "хуесос",
    "ашаурак",
    "идиотина",
    "тупица",
    "затупок",
    "душнила",
];

pub const POSITIVE_EMOJIS: &[&str] = &[
    "👍", "🔥", "💖", "✨", "💪", "🎉", "🥳", "🌟", "❤️", "👏", "👑", "🫶", "🕺", "💃", "🚀", "🥇",
    "🥳",
];

pub const NEGATIVE_EMOJIS: &[&str] = &[
    "👎", "💩", // говно
    "🤢", // тошнота
    "🤮", // рвота
    "😤", // злость
    "🥴", // поплыл
    "🙄", // закат глаз
    "😬", // неловко
    "🚽", // унитаз
    "⚰️", // гроб
    "🥵", // перегрев
    "🥶", // перемёрз
    "🤕", // ушибленный
    "🤒", // больной
    "💀", // умер
];

pub fn negative_word() -> Result<String> {
    NEGATIVE
        .choose(&mut rand::thread_rng())
        .ok_or(anyhow!("random"))
        .map(|s| s.to_string())
}
