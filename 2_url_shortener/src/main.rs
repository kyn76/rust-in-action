#[macro_use]
extern crate rocket;
mod database;

use database::requests::UrlsRequest;
use database::responses::Urls;
use database::{get_all_urls, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};


#[get("/urls")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<Urls>>> {
    let urls = get_all_urls(pool).await?;
    println!("Urls: {:?}", urls);
    Ok(Json(urls))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("urls.sqlite")
        .await
        .expect("Couldn't connect to sqlite database");

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}