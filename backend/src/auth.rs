use diesel::{prelude::*, result::Error};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::users::User;
use crate::util::verify_password;
use crate::DollopDbConnection;

#[derive(FromForm, Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginMessage<'r> {
    status_code: i16,
    status: &'r str,
    message: String,
}

#[post("/login", format = "json", data = "<login_info>")]
pub async fn login<'r>(
    database_connection: DollopDbConnection,
    login_info: Json<LoginInfo>,
    cookies: &CookieJar<'_>,
) -> Json<LoginMessage<'r>> {
    let login_username = login_info.username.clone();
    let login_password = login_info.password.clone();

    let result: Result<User, Error> = database_connection
        .run(move |connection| {
            users::table
                .select((users::id, users::username, users::password))
                .filter(users::username.eq(&login_info.username))
                .first(connection)
        })
        .await;

    if let Ok(user) = &result {
        if verify_password(&login_password, &user.password) {
            cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));

            return Json(LoginMessage {
                status_code: 200,
                status: "OK",
                message: String::from("Successfully logged in"),
            });
        } else {
            return Json(LoginMessage {
                status_code: 401,
                status: "Unauthorized",
                message: String::from("Wrong password"),
            });
        }
    }

    return Json(LoginMessage {
        status_code: 401,
        status: "Unauthorized",
        message: format!("Couldn't find a user with username {}", login_username),
    });
}

#[post("/logout")]
pub fn logout<'r>(cookies: &CookieJar<'_>) -> Json<LoginMessage<'r>> {
    if cookies.get_private("user_id").is_none() {
        return Json(LoginMessage {
            status_code: 400,
            status: "Bad Request",
            message: String::from("You don't have that cookie"),
        });
    }

    cookies.remove_private(Cookie::named("user_id"));

    Json(LoginMessage {
        status_code: 200,
        status: "OK",
        message: String::from("Successfully logged out. "),
    })
}
