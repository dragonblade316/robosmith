use sqlx::{pool::{Pool, PoolOptions}, query, query_as, sqlite::SqlitePoolOptions, SqlitePool};

//This must be run at the start of the program to ensure integrity of the qotd database.
pub async fn init_qotd(db_pool: &SqlitePool) {
    query("CREATE TABLE IF NOT EXISTS Guilds (
        id INT PRIMARY KEY AUTO INCREMENT,
        guild_id BIGINT NOT NULL,
        question NVARCHAR(4000) NOT NULL
    );").execute(db_pool).await.expect("failed to create qotd table");


}







