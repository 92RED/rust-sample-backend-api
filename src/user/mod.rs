use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use self::model::User;
use super::db::Conn as DbConn;

pub mod model;
pub mod schema;

pub fn mount(basepath: &str, rocket: rocket::Rocket) -> rocket::Rocket {
    let path = format!("{}{}{}", basepath, "/", "User");
    rocket.mount(&path, routes![get_all, new_user, find_user, hello])
}

#[get("/users", format = "text/html")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<User>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[get("/user/<name>", format = "text/html")]
pub fn find_user(conn: DbConn, name: &RawStr) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(name, &conn),
    }))
}

#[get("/hello/<name>")]
pub fn hello(name: &RawStr) -> Json<Value> {
    Json(json!({
    "status": 200,
    "result": format!("Hello, {}!", name.as_str()),
    }))
}
