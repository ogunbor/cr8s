use crate::models::{NewRustacean, Rustacean, User};
use crate::rocket_routes::{DbConn, server_error};
use crate::repositories::RustaceanRepository;

use rocket::serde::json::{Json, serde_json::json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::put("/rustaceans/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(mut db: Connection<DbConn>, id: i32, rustacean: Json<Rustacean>, _user: User) -> Result<Value, Custom<Value>> {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    }).await
}
