use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UrlsRequest {
    pub full_url: String,
    pub short_url: String,
}