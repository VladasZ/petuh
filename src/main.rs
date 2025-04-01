use teloxide::prelude::*;
use teloxide::types::Message;
use rand::prelude::SliceRandom;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Приветствие.")]
    K,
    #[command(description = "Уважение Максиму.")]
    M,
    #[command(description = "Уважение Роме.")]
    R,
    #[command(description = "Доброе утро.")]
    Gm,
    #[command(description = "Узнать кто петух.")]
    Kto,
    #[command(description = "Бросить петушиный кубик.")]
    Kub,
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

    dbg!(&msg);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::K => {
            bot.send_message(msg.chat.id, "Кукарекуууу я петушара!!!").await?;
        }
        Command::M => {
            bot.send_message(msg.chat.id, "Максим Пятушара!!!! 🐓🐓🐓🐓🐓").await?;
        }
        Command::R => {
            bot.send_message(msg.chat.id, "Рома каблук петушиный 👠").await?;
        }
        Command::Gm => {
            bot.send_message(msg.chat.id, "Доброе утро петушары ебаные!").await?;
        }
        Command::Kto => {
            let names = ["Максим", "Владик", "Владас", "Рома", "Настя", "Денис"];
            let name = names.choose(&mut rand::thread_rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
        Command::Kub => {
            bot.send_dice(msg.chat.id).await?;
        }
    }
    Ok(())
}


async fn handle_text(bot: Bot, msg: Message) -> ResponseResult<()> {

    dbg!(&msg);

    if let Some(text) = msg.text() {
        if text == "Кто петух?" {
            let names = ["Максим", "Владик", "Владас", "Рома", "Настя", "Денис"];
            let name = names.choose(&mut rand::thread_rng()).unwrap();
            let reply = format!("{name} — петух! 🐓");
            bot.send_message(msg.chat.id, reply).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Load the bot token from environment variable
    dotenv::dotenv().ok();
    log::info!("Starting Telegram bot...");

    let bot = Bot::from_env();

    // Build dispatcher with both command and message handlers
    let handler = dptree::entry()
        .branch(Update::filter_message()
            .filter_command::<Command>()
            .endpoint(handle_command))
        .branch(Update::filter_message()
            .endpoint(handle_text));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
