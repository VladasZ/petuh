use anyhow::{Result, anyhow};
use rand::prelude::IndexedRandom;

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
    "клоун",
    "еблан",
    "далбоёб",
    "чушпан",
    "обмудок",
    "хуеплёт",
    "жополиз",
    "мудозвон",
    "говноед",
    "пидор",
    "чмо",
    "ошибка природы",
    "придурок",
    "гандон",
    "тварь тупая",
    "чепушила",
    "недоумок",
    "кретин",
    "гнида",
    "унылое гавно",
    "мразь",
    "улшепок",
    "пиздаватая хуета",
    "обмудская залупа",
    "ничтожество",
    "позор для родителей",
    "мошоночный сосальщик",
    "уебище лесное",
    "грязный пидорас",
];

pub const NEGATIVE_ADJ: &[&str] = &[
    "вонючий",
    "дряхлый",
    "тупой",
    "обоссаный",
    "ничтожный",
    "ебаный",
    "гнилой",
    "мерзкий",
    "облезлый",
    "безмозглый",
    "сраный",
    "ссаный",
    "парашный",
    "чмошный",
    "пиздаватый",
    "залупный",
    "уёбищный",
    "опущенный",
    "ебанутый",
    "ёбнутый на всю тупую башку",
    "злоебучий",
    "дурацкий",
    "трухлявый",
    "жалкий",
    "грязный",
    "драный в сраку неграми",
];

pub const POSITIVE_EMOJIS: &[&str] = &[
    "👍", "🔥", "💖", "✨", "💪", "🎉", "🥳", "🌟", "❤️", "👏", "👑", "🫶", "🕺", "💃", "🚀", "🥇", "🥳",
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
        .choose(&mut rand::rng())
        .ok_or(anyhow!("random"))
        .map(ToString::to_string)
}
