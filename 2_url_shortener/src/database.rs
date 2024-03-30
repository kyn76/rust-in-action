use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::Urls;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;


pub async fn get_all_urls(pool: &Pool<Sqlite>) -> DBResult<Vec<Urls>> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let urls = sqlx::query_as::<_, Urls>(
        r#"
        SELECT full_url, short_url 
        FROM urls 
        "#
    )
        .fetch_all(&mut connection)
        .await?;
        Ok(urls)
}