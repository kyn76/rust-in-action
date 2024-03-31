#[macro_use]
extern crate rocket;
mod database;

use database::requests::UrlPairRequest;
use database::responses::UrlPair;
use database::{get_all_url_pairs, get_full_url, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/showall")]
async fn showall(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<UrlPair>>> {
    let urls = get_all_url_pairs(pool).await?;
    println!("Url Pairs: {:?}", urls);
    Ok(Json(urls))
}

#[get("/<short_url>")]
async fn redirect(short_url: String, pool: &State<Pool<Sqlite>>) -> DBResult<Json<String>> {
    let full_url = get_full_url(&short_url, pool).await?;
    println!("Url: {:?}", full_url);
    Ok(Json(full_url))
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // connect to DB
    let pool = SqlitePool::connect("urls.sqlite")
        .await
        .expect("Couldn't connect to sqlite database");

    // launch server
    let _rocket = rocket::build()
        .mount("/", routes![index, showall, redirect])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}