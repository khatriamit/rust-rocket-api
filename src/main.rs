#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustaceans,
                rocket_routes::rustaceans::update_rustaceans,
                rocket_routes::rustaceans::create_rustaceans,
                rocket_routes::rustaceans::delete_rustaceans,
                rocket_routes::crates::get_crates,
                rocket_routes::crates::view_crates,
                rocket_routes::crates::update_crates,
                rocket_routes::crates::create_crates,
                rocket_routes::crates::delete_crates,
            ],
        )
        .attach(rocket_routes::DbConn::fairing())
        .launch()
        .await;
}
