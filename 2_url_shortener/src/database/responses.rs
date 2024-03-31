use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UrlPair {
    pub full_url: String,
    pub short_url: String,
}