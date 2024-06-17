use std::{fs, path::PathBuf, time::Duration};

use const_format::formatcp;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

const CONN_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const DB_URL: &str = "postgres://dev_user:dev_password@localhost/dev_db";

const SQL_DIR: &str = "sql/dev";
const SQL_EXTENSION: &str = "sql";
const RECREATE_DB_SQL: &str = formatcp!("{SQL_DIR}/00__recreate_db.{SQL_EXTENSION}");

const TIMEOUT_MILLIS: Duration = Duration::from_millis(500);

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - dev_db::init", "DEV");

    {
        let pool = new_pool(CONN_URL).await?;
        exec(&pool, RECREATE_DB_SQL).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let pool = new_pool(DB_URL).await?;
    for opt_path in paths {
        if let Some(path) = opt_path.to_str() {
            let path = path.replace('\\', "/");
            if path.ends_with(format!(".{SQL_EXTENSION}").as_str()) && path != RECREATE_DB_SQL {
                exec(&pool, &path).await?;
            }
        }
    }

    Ok(())
}

async fn exec(pool: &Pool<Postgres>, file_path: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - dev_db::exec: {file_path}", "DEV");

    let sql = fs::read_to_string(file_path)?;
    let statements: Vec<&str> = sql.split(';').collect();
    for statement in statements {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}

async fn new_pool(db_conn_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(TIMEOUT_MILLIS)
        .connect(db_conn_url)
        .await
}
