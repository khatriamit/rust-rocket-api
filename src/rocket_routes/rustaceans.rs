use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{NewRustacean, Rustacean},
    repositories::RustaceanRepository,
    rocket_routes::DbConn,
};

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        RustaceanRepository::find_multiple(conn, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustaceans(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustaceanRepository::find(conn, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustaceans(
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustaceanRepository::create(conn, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustaceans(
    db: DbConn,
    id: i32,
    rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustaceanRepository::save(conn, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustaceans(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |conn| {
        RustaceanRepository::delete(conn, id)
            .map(|_| NoContent)
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}
