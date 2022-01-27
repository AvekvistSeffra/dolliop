#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;

mod cors;
mod errors;
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
}
