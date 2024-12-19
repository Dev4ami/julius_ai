mod api_julius;
mod command;

use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN harus diatur di file .env");
        
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    println!("server started");
    let bot = Bot::new(token.clone());

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
        .branch(dptree::endpoint(handle_non_command_message));
        
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    Start,
    Help,
    New,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let _ = match cmd {
        Command::Start => command::start(bot, msg).await,
        Command::Help => command::help(bot, msg).await,
        Command::New => command::new(bot, msg).await,
    };
    Ok(())
}

async fn handle_non_command_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let response = api_julius::asking_ai(msg.text().unwrap().to_string()).await;
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}



