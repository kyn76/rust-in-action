#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use rocket::data::Outcome;

// DB configuration
#[Outcome::database("sqlite_db")]
struct UrlsDbConnection(diesel::SqliteConnection);


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
fn main() -> _ {
    // // init url database in memory
    // let connection = sqlite::open(":memory:").unwrap();
    // let query = "
    // CREATE TABLE users (name TEXT, age INTEGER);
    // INSERT INTO users VALUES ('alice', 42);
    // INSERT INTO users VALUES ('bob', 69);
    // ";
    // connection.execute(query).unwrap();


    rocket::ignite()
        // .mount("/", routes![index, google])
        .attach(UrlsDbConnection::fairing())
        .launch();
}
