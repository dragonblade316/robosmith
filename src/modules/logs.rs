use std::ops::Range;

use poise::serenity_prelude::{self as serenity, CacheHttp, GuildId, Mention, UserId};
use songbird::SerenityInit;
//
use sqlx::{pool::{Pool, PoolOptions}, prelude::FromRow, query, query_as, SqlitePool};
use crate::{Data, Context, Error};

#[derive(FromRow, Debug, Clone)]
struct Entry {
    id: i32,
    guild_id: i64,
    channel_id: i64,
    message_id: i64,
    user_id: i64,
    username: String,
    display_name: String,
    message: String
}

async fn logs_init(db_pool: &SqlitePool) {
    query("CREATE TABLE IF NOT EXISTS Logs (
        id INT AUTO_INCREMENT PRIMARY KEY,
        guild_id BIGINT NOT NULL,
        channel_id BIGINT NOT NULL,
        message_id BIGINT NOT NULL,
        user_id BIGINT NOT NULL,
        username NVARCHAR(32) NOT NULL,
        display_name NVARCHAR(32),
        message NVARCHAR(4000) NOT NULL
    );").execute(db_pool).await.expect("failed to create Guilds table");
}

#[poise::command(slash_command, prefix_command)]
pub async fn user_messages(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
    #[description = "Number of messages"] amount: i64
) -> Result<(), Error> {
    
    let data: Vec<Entry> = match user {
        Some(u) => query_as("SELECT * FROM Logs WHERE user_id=$1 ORDER BY id DESC LIMIT $2;").bind(u.id.get() as i64).bind(amount).fetch_all(&ctx.data().database).await,
        None => query_as("SELECT * FROM Logs WHERE user_id!=849120659439484978 ORDER BY id DESC LIMIT $1;").bind(amount).fetch_all(&ctx.data().database).await,
    }.expect("thing");

    for i in data {
        println!("{:?}", i.clone());
        if i.message == "" || i.user_id == 849120659439484978 {continue}

        let user = Mention::User(UserId::new(i.user_id as u64));
        
        ctx.say(format!("{} said: \n{}", user, i.message)).await.unwrap();   
    }

    // let u = user.as_ref().unwrap_or_else(|| ctx.author());
    // let response = format!("{}'s account was created at {}", u.name, u.created_at());
    // ctx.say(response).await?;
    Ok(())
}

