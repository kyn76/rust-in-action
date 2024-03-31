#[macro_use]
extern crate rocket;
mod database;

use database::requests::UrlPairRequest;
use database::responses::UrlPair;
use database::{delete_url_pair, get_all_url_pairs, get_full_url, insert_url_pair, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/urls")]
async fn showall(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<UrlPair>>> {
    let urls = get_all_url_pairs(pool).await?;
    println!("Url Pairs: {:?}", urls);
    Ok(Json(urls))
}

#[post("/url", format = "json", data = "<url_pair>")]
async fn create_redirection(url_pair: Json<UrlPairRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<String>> {
    insert_url_pair(pool, &url_pair.full_url, &url_pair.short_url).await?;
    let response = format!("Short url {} created to redirect to {}", &url_pair.short_url, &url_pair.full_url);
    Ok(Json(response))
}


#[get("/url/<short_url>")]
async fn redirect(short_url: String, pool: &State<Pool<Sqlite>>) -> DBResult<Json<String>> {
    let full_url = get_full_url(pool, &short_url).await?;
    println!("Url: {:?}", full_url);
    Ok(Json(full_url))
}

#[delete("/url/<short_url>")]
async fn delete_redirection(short_url: String, pool: &State<Pool<Sqlite>>) -> DBResult<Json<String>> {
    delete_url_pair(pool, &short_url).await?;
    let response = format!("Redirection with short url '{}' deleted successfully", short_url);
    Ok(Json(response))
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // connect to DB
    let pool = SqlitePool::connect("urls.sqlite")
        .await
        .expect("Couldn't connect to sqlite database");

    // launch server
    let _rocket = rocket::build()
        .mount("/", routes![index, showall, create_redirection, redirect, delete_redirection])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}