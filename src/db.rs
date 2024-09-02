use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};
use std::time::Duration;
use dotenv;

const DB_USER_KEY: &str = "DB_USER";
const DB_PASSWORD_KEY: &str = "DB_PASSWORD";

pub async fn init() -> Result<Pool<Postgres>, Error> {
    let db_user = dotenv::var(DB_USER_KEY)
        .unwrap_or_else(|_| std::env::var(DB_USER_KEY).expect(format!("{DB_USER_KEY} must be set in env").as_str())
    );
    
    let db_password = dotenv::var(DB_PASSWORD_KEY)
        .unwrap_or_else(|_| std::env::var(DB_PASSWORD_KEY).expect(format!("{DB_PASSWORD_KEY} must be set in env").as_str())
    );

    let connection_string = format!("postgres://postgres.{}:{}@aws-0-eu-central-1.pooler.supabase.com:5432/postgres", db_user, db_password);

    // user postgres.{} on db postgres
    PgPoolOptions::new()
        .min_connections(5)
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect(connection_string.as_str())
        .await
}