use crate::api_julius;

use teloxide::{prelude::*};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Server Onlline").await?;
    Ok(())
}


pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_data = help_data().await;
    bot.send_message(msg.chat.id, help_data).await?;
    Ok(())
}


async fn help_data() -> String {
    format!("
This Bot For Private Ai (Test Mode)

/start  ==> Start Bot
/help  ==> Bantuan
/new  ==> Renew Chat
/draw  ==> Generate Image

")}


pub async fn new(bot: Bot, msg: Message) -> ResponseResult<()> {
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    let pool = MySqlPool::connect(&database_url).await
        .expect("Failed to create pool.");

    let demo_id = api_julius::generate_demo_id().await;
    if demo_id.0 == true {
        let conversation_id = api_julius::generate_conversation_id(&demo_id.1).await;
        if conversation_id.0 == true {
            let query = sqlx::query("INSERT INTO user_data (demo_id, conversation_id) VALUES (?, ?)")
                .bind(&demo_id.1)
                .bind(&conversation_id.1);
            query.execute(&pool).await.expect("Failed to execute query.");
            bot.send_message(msg.chat.id, "Chat Berhasil diperbaharui").await?;
        } else {
            bot.send_message(msg.chat.id, "Gagal Mendapatkan Conversation ID").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "Gagal Mendapatkan User ID").await?;
    }
    Ok(())
}


