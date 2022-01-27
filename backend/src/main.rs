#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;

mod auth;
mod cors;
mod entries;
mod errors;
mod items;
mod lists;
mod schema;
mod users;
mod util;

#[database("dollop")]
pub struct DollopDbConnection(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DollopDbConnection::fairing())
        .attach(cors::Cors)
        .register(
            "/",
            catchers![
                errors::bad_request,
                errors::unauthorized,
                errors::forbidden,
                errors::not_found
            ],
        )
        .mount("/", routes![])
        .mount("/auth", routes![auth::login, auth::logout])
        .mount(
            "/users",
            routes![
                users::create,
                users::list,
                users::read,
                users::update,
                users::delete
            ],
        )
        .mount(
            "/items",
            routes![
                items::create,
                items::list,
                items::read,
                items::update,
                items::delete
            ],
        )
}
