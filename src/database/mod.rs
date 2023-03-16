use sqlx::{postgres::PgPoolOptions, PgPool};

pub(crate) async fn open_connection(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
