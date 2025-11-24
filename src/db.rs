use sqlx::{
    postgres::{PgPoolOptions},
    PgPool, Row,
};
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn increment(db: &PgPool, key: &str) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO kv_store (key, value)
        VALUES ($1, $2)
        ON CONFLICT (key)
        DO UPDATE SET value = kv_store.value + EXCLUDED.value
        RETURNING value
        "#,
    )
    .bind(key)
    .bind(1) // is binding even necesarry? sql injections do be scary
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("value"))
}

pub async fn read_value(db: &PgPool, key: &str) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT value
        FROM kv_store
        WHERE key = $1;
        "#
    )
    .bind(key)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("value"))
}

pub async fn update_value(db: &PgPool, key: &str, val: i64) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO kv_store (key, value)
        VALUES ($1, $2)
        ON CONFLICT (key)
        DO UPDATE SET value = EXCLUDED.value
        RETURNING value
        "#,
    )
    .bind(key)
    .bind(val)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("value"))
}

pub async fn delete_key(db: &PgPool, key: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM kv_store
        WHERE key = $1
        "#,
    )
    .bind(key)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}