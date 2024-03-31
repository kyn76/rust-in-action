use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UrlPairRequest {
    pub full_url: String,
    pub short_url: String,
}