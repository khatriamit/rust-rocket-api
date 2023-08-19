use diesel::PgConnection;
use rocket_sync_db_pools::database;

pub mod crates;
pub mod rustaceans;

#[database("postgres")]
pub struct DbConn(PgConnection);
