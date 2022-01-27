use diesel::{prelude::*, Insertable, Queryable};
use rocket::response::{
    status::{Accepted, Created},
    Debug,
};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::util::hash_password;
use crate::DollopDbConnection;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Clone, Serialize, Deserialize, Queryable, AsChangeset, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct QueryableUser {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct InsertableUser {
    username: String,
    password: String,
}

#[post("/", format = "json", data = "<user>")]
pub async fn create(
    database_connection: DollopDbConnection,
    user: Json<InsertableUser>,
) -> Result<Created<Json<QueryableUser>>> {
    let user = user.clone();

    let password = match hash_password(&user.password) {
        Ok(password) => password,
        Err(_) => String::new(),
    };

    let new_user = InsertableUser {
        username: user.username,
        password: password,
    };

    database_connection
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(conn)
        })
        .await?;

    let created_user: QueryableUser = database_connection
        .run(move |conn| {
            users::table
                .select((users::id, users::username))
                .order(users::id.desc())
                .first(conn)
        })
        .await?;

    Ok(Created::new("/").body(Json(created_user)))
}

#[get("/")]
pub async fn list(database_connection: DollopDbConnection) -> Result<Json<Vec<QueryableUser>>> {
    let users = database_connection
        .run(move |connection| {
            users::table
                .select((users::id, users::username))
                .load(connection)
        })
        .await?;

    Ok(Json(users))
}

#[get("/<id>")]
pub async fn read(database_connection: DollopDbConnection, id: i32) -> Option<Json<QueryableUser>> {
    database_connection
        .run(move |connection| {
            users::table
                .select((users::id, users::username))
                .filter(users::id.eq(id))
                .first(connection)
        })
        .await
        .map(Json)
        .ok()
}

#[post("/<id>", format = "json", data = "<user>")]
pub async fn update(
    database_connection: DollopDbConnection,
    id: i32,
    user: Json<InsertableUser>,
) -> Result<Accepted<Json<i32>>> {
    let password = match hash_password(&user.password) {
        Ok(password) => password,
        Err(_) => String::new(),
    };

    database_connection
        .run(move |conn| {
            diesel::update(users::table.filter(users::id.eq(id)))
                .set((
                    users::username.eq(&user.username),
                    users::password.eq(&password),
                ))
                .execute(conn)
        })
        .await?;

    Ok(Accepted(Some(Json(id))))
}

#[delete("/<id>")]
pub async fn delete(database_connection: DollopDbConnection, id: i32) -> Result<Option<()>> {
    let affected = database_connection
        .run(move |conn| {
            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}
