use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::migrate::Migrator;
use std::str::FromStr;
use std::path::Path;

mod ledger;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database file if it doesn't exist
    if !database_url.starts_with("sqlite:") {
        panic!("Only sqlite is supported");
    }
    
    let db_path_str = database_url.trim_start_matches("sqlite:");
    // handle relative paths or memory
    if db_path_str != ":memory:" {
         let path = Path::new(db_path_str);
         if !path.exists() {
             println!("Database file not found. Creating...");
             std::fs::File::create(path)?;
         }
    }

    let pool = SqlitePool::connect(&database_url).await?;

    println!("Running migrations...");
    MIGRATOR.run(&pool).await?;
    println!("Migrations passed.");
    
    // Verify table existence
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='event_log'")
        .fetch_one(&pool)
        .await?;

    if row.0 == 1 {
        println!("VERIFICATION: event_log table exists.");
    } else {
        panic!("VERIFICATION FAILED: event_log table missing.");
    }

    Ok(())
}