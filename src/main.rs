use poise::serenity_prelude::{self as serenity, CacheHttp, GuildId};
use songbird::SerenityInit;
use sqlx::{pool::{Pool, PoolOptions}, query, query_as, SqlitePool};

mod modules;

struct Data {
    database: SqlitePool
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    //.env file
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let mut intents = serenity::GatewayIntents::non_privileged();
    intents.set(serenity::GatewayIntents::MESSAGE_CONTENT, true);
    println!("{}",intents.message_content());
    

    let db_options = sqlx::sqlite::SqliteConnectOptions::new()
    .filename("mainbase.db")
    .create_if_missing(true);
    
    //database setup
    let db_pool = SqlitePool::connect_with(db_options).await.expect("failed to connect to db");


    //TODO: maybe move this to init functions
    query("CREATE TABLE IF NOT EXISTS Guilds (
        id INT PRIMARY KEY,
        guild_id VARCHAR(30) NOT NULL
    );").execute(&db_pool).await.expect("failed to create Guilds table");
    println!("users table verified");

    query("CREATE TABLE IF NOT EXISTS Logs (
        id INT AUTO_INCREMENT PRIMARY KEY,
        guild_id BIGINT NOT NULL,
        channel_id BIGINT NOT NULL,
        message_id BIGINT NOT NULL,
        user_id BIGINT NOT NULL,
        username NVARCHAR(32) NOT NULL,
        display_name NVARCHAR(32),
        message NVARCHAR(4000) NOT NULL
    );").execute(&db_pool).await.expect("failed to create Guilds table");
    println!("logs table verified");

    modules::qotd::init_qotd(&db_pool);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some("!".into()), 
                case_insensitive_commands: true,
                ..Default::default()
            },

            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },

            commands: vec![age(), modules::logs::user_messages()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    database: db_pool
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {

            let _ = query("INSERT INTO Logs (guild_id, channel_id, message_id, user_id, username, display_name, message) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(new_message.guild_id.unwrap_or(GuildId::new(849120659439484978)).get() as i64)
                .bind(new_message.channel_id.get() as i64)
                .bind(new_message.id.get() as i64)
                .bind(new_message.author.id.get() as i64)
                .bind(new_message.author.name.clone())
                .bind(new_message.author.display_name())
                //this might be slightly expensive
                .bind(new_message.content.clone())
                .execute(&data.database).await;

            println!("logger run");
            println!("text {}", new_message.content);

        }
        _ => {}
    }
    Ok(())
}

