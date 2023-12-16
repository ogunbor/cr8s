use crate::models::{Crate, NewCrate, User};
use crate::rocket_routes::{DbConn, server_error};
use crate::rocket_routes::{DbConn, EditorUser, server_error};
use rocket::serde::json::{Json, serde_json::json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>, _user: EditorUser) -> Result<Custom<Value>, Custom<Value>> {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>, _user: EditorUser) -> Result<Value, Custom<Value>> {
        CrateRepository::update(c, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    }).await
}
