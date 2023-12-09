use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);


pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

