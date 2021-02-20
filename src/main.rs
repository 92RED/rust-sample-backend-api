#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::process::Command;

use dotenv::dotenv;

mod db;
mod user;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let path = "/api/v1/";
    let pool = db::init_pool(database_url);
    let mut rocket = rocket::ignite().manage(pool);

    rocket = user::mount(path, rocket);

    return rocket
}

fn main() {
    let _output = Command::new("sh")
        .arg("-c")
        .arg("cd ui && npm start")
        .spawn()
        .expect("Failed to start UI Application");
    rocket().launch();
}
