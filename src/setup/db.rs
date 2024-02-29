use std::env;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use log::{error, LevelFilter};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use migration::MigratorTrait;

#[derive(Debug, Default)]
struct DbConfig {
    db_url: String,
    max_conn: u32,
    min_conn: u32,
    conn_timeout: u64,
    acquire_timeout: u64,
    idle_time: u64,
    max_lifetime: u64,
    sql_logging: bool,
    sql_log_level: LevelFilter,
    schema: String,
}

#[derive(Clone,Debug)]
pub struct DbConn(pub Arc<DatabaseConnection>);

impl DbConn {
    fn get(self) -> Arc<DatabaseConnection> {
        self.0.clone()
    }
}


impl DbConfig {
    fn default() -> Self {
        let common_time = "30";
        Self {
            db_url: get_str_env("DATABASE_URL", ""),
            max_conn: get_u32_env("DB_MAX_CONN", "10"),
            min_conn: get_u32_env("DB_MIN_CONN", "2"),
            conn_timeout: get_u64_env("DB_CONN_TIMEOUT", common_time),
            acquire_timeout: get_u64_env("DB_ACQUIRE_TIMEOUT", "10"),
            idle_time: get_u64_env("DB_IDLE_TIME", common_time),
            max_lifetime: get_u64_env("DB_ACQUIRE_TIMEOUT", common_time),
            sql_logging: get_bool_env("DB_LOGGING", "false"),
            sql_log_level: get_log_level(),
            schema: get_str_env("DB_SCHEMA", ""),
        }
    }
}

fn get_str_env(var: &str, default_val: &str) -> String {
    env::var(var).unwrap_or(default_val.to_string())
}
fn get_u32_env(var: &str, default_val: &str) -> u32 {
    env::var(var)
        .unwrap_or(default_val.to_string())
        .parse()
        .unwrap()
}
fn get_u64_env(var: &str, default_val: &str) -> u64 {
    env::var(var)
        .unwrap_or(default_val.to_string())
        .parse()
        .unwrap()
}
fn get_log_level() -> LevelFilter {
    let log_level = env::var("DB_LOG_LEVEL").unwrap_or("info".to_string());
    log::LevelFilter::from_str(&log_level).unwrap_or(LevelFilter::Info)
}

fn get_bool_env(var: &str, default_val: &str) -> bool {
    env::var(var)
        .unwrap_or(default_val.to_string())
        .parse()
        .unwrap()
}

pub async fn init_db()->DbConn{
    let db_conn = get_db_conn().await;
    migration::Migrator::up(&db_conn, None)
        .await
        .expect("migration error");
    let db_conn = DbConn(Arc::new(db_conn));
    db_conn
}

///get db connect
pub async fn get_db_conn() -> DatabaseConnection {
    let config = DbConfig::default();
    if config.db_url.is_empty() {
        error!("db url  is empty");
        exit(5001)
    }
    let mut opt = ConnectOptions::new(&config.db_url);
    opt.max_connections(config.max_conn)
        .min_connections(config.min_conn)
        .connect_timeout(Duration::from_secs(config.conn_timeout))
        .acquire_timeout(Duration::from_secs(config.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.idle_time))
        .max_lifetime(Duration::from_secs(config.max_lifetime))
        .sqlx_logging(config.sql_logging)
        .sqlx_logging_level(config.sql_log_level)
        .set_schema_search_path(&config.schema); // Setting default PostgreSQL schema

    match Database::connect(opt).await {
        Ok(db) => {return  db}
        Err(e) => {
            error!("db conn error {}",e);
            exit(5002)
        }
    }
}
