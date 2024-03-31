use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::UrlPair;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;


pub async fn get_all_url_pairs(pool: &Pool<Sqlite>) -> DBResult<Vec<UrlPair>> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let url_pairs = sqlx::query_as::<_, UrlPair>(
        r#"
        SELECT full_url, short_url 
        FROM urls 
        "#
    )
        .fetch_all(&mut connection)
        .await?;
        Ok(url_pairs)
}

pub async fn get_full_url(pool: &Pool<Sqlite>, short_url: &str) -> DBResult<String> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let url_pair = sqlx::query!(
        r#"
        SELECT full_url, short_url
        FROM urls 
        WHERE short_url = ?
        "#,
        short_url
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(url_pair.full_url.unwrap())
}

pub async fn insert_url_pair(
    pool: &Pool<Sqlite>,
    full_url: &String,
    short_url: &String,
) -> DBResult<()> {
    let mut connection = pool
        .acquire()
        .await?;
    sqlx::query!(
        r#"
        INSERT INTO urls VALUES (?, ?);
        "#,
        full_url,
        short_url
    )
        .execute(&mut connection)
        .await?;
        Ok(())
}

pub async fn delete_url_pair(
    pool: &Pool<Sqlite>,
    short_url: &String,
) -> DBResult<()> {
    let mut connection = pool
        .acquire()
        .await?;
    sqlx::query!(
            r#"
        DELETE FROM urls
        WHERE short_url = ?;
        "#,
        short_url
    )
        .execute(&mut connection)
        .await?;
        Ok(())
}