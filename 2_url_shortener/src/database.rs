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

pub async fn get_full_url(short_url: &str, pool: &Pool<Sqlite>) -> DBResult<String> {
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