use rocket::serde::json::Json;
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage<'r> {
    status_code: i16,
    status: &'r str,
    message: String,
}

#[catch(400)]
pub fn bad_request<'r>(req: &Request) -> Json<ErrorMessage<'r>> {
    println!("{}", req.uri().path());

    Json(ErrorMessage {
        status_code: 400,
        status: "Bad Request",
        message: format!(
            "We couldn't get that, try fixing something at your end. Path accessed: {}",
            req.uri().path()
        ),
    })
}

#[catch(401)]
pub fn unauthorized<'r>(req: &Request) -> Json<ErrorMessage<'r>> {
    println!("{}", req.uri().path());

    Json(ErrorMessage {
        status_code: 401,
        status: "Unauthorized",
        message: format!(
            "You're not logged in and therefore not allowed to view this item: {}",
            req.uri().path()
        ),
    })
}

#[catch(403)]
pub fn forbidden<'r>(req: &Request) -> Json<ErrorMessage<'r>> {
    println!("{}", req.uri().path());

    Json(ErrorMessage {
        status_code: 403,
        status: "Forbidden",
        message: format!(
            "You don't have permissions to view this item: {}",
            req.uri().path()
        ),
    })
}

#[catch(404)]
pub fn not_found<'r>(req: &Request) -> Json<ErrorMessage<'r>> {
    println!("{}", req.uri().path());

    Json(ErrorMessage {
        status_code: 404,
        status: "Not Found",
        message: format!("The requested URL was not found: {}", req.uri().path()),
    })
}
