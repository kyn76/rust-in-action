#[macro_use] extern crate rocket;
use rocket_sync_db_pools::{database, diesel};


#[database("sqlite_db")]
struct UrlsDbConnection(diesel::SqliteConnection);

#[derive(Queryable)]
struct Urls {
    full_url: String,
    short_url: String,
}


fn load_from_db(connection: &diesel::SqliteConnection) -> String {
    use crate::schema::urls::dsl::*;

    let results = urls
}

#[get("/")]
async fn index(mut connection: UrlsDbConnection) -> &'static str {
    connection.run(|c| load_from_db(c)).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(UrlsDbConn::fairing())
}

// // Routes
// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[get("/google")]
// fn google(connection: UrlsDbConnection) {
//     // access bdd item here
//     let query = "SELECT * FROM urls;";
//     connection
//         .iterate(query, |row| {
//             let full_url: String = row.get(0).unwrap();
//             println!("Full: {}", full_url);
//         true
//         })
//         .unwrap();
// }

// // #[get("/showall")]
// fn showall() -> &'static str {
//     "Hello, showall!"
// }

// #[launch]
// fn main() -> _ {

//     rocket::ignite()
//         // .mount("/", routes![index, google])
//         .attach(UrlsDbConnection::fairing())
//         .launch();
// }
