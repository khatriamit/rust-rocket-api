use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
    rocket_routes::DbConn,
};

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        CrateRepository::find_multiple(conn, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crates(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::find(conn, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crates(db: DbConn, new_crate: Json<NewCrate>) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::create(conn, new_crate.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crates(
    db: DbConn,
    id: i32,
    a_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::save(conn, id, a_crate.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crates(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::delete(conn, id)
            .map(|_| NoContent)
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}
