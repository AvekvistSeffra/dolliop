use diesel::{prelude::*, Insertable, Queryable};
use rocket::response::{
    status::{Accepted, Created},
    Debug,
};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::schema::items;
use crate::DollopDbConnection;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Clone, Serialize, Deserialize, Queryable, AsChangeset, Identifiable)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "items"]
pub struct InsertableItem {
    name: String,
    price: f32,
}

#[post("/", format = "json", data = "<item>")]
pub async fn create(
    database_connection: DollopDbConnection,
    item: Json<InsertableItem>,
) -> Result<Created<Json<Item>>> {
    database_connection
        .run(move |conn| {
            diesel::insert_into(items::table)
                .values(&*item)
                .execute(conn)
        })
        .await?;

    let created_item: Item = database_connection
        .run(move |conn| {
            items::table
                .select((items::id, items::name, items::price))
                .order(items::id.desc())
                .first(conn)
        })
        .await?;

    Ok(Created::new("/").body(Json(created_item)))
}

#[get("/")]
pub async fn list(database_connection: DollopDbConnection) -> Result<Json<Vec<Item>>> {
    let items = database_connection
        .run(move |connection| {
            items::table
                .select((items::id, items::name, items::price))
                .load(connection)
        })
        .await?;

    Ok(Json(items))
}

#[get("/<id>")]
pub async fn read(database_connection: DollopDbConnection, id: i32) -> Option<Json<Item>> {
    database_connection
        .run(move |connection| {
            items::table
                .select((items::id, items::name, items::price))
                .filter(items::id.eq(id))
                .first(connection)
        })
        .await
        .map(Json)
        .ok()
}

#[post("/<id>", format = "json", data = "<item>")]
pub async fn update(
    database_connection: DollopDbConnection,
    id: i32,
    item: Json<InsertableItem>,
) -> Result<Accepted<Json<i32>>> {
    database_connection
        .run(move |conn| {
            diesel::update(items::table.filter(items::id.eq(id)))
                .set((items::name.eq(&item.name), items::price.eq(&item.price)))
                .execute(conn)
        })
        .await?;

    Ok(Accepted(Some(Json(id))))
}

#[delete("/<id>")]
pub async fn delete(database_connection: DollopDbConnection, id: i32) -> Result<Option<()>> {
    let affected = database_connection
        .run(move |conn| {
            diesel::delete(items::table)
                .filter(items::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}
