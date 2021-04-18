#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
//extern crate diesel;


use rocket_contrib::databases::diesel::{self, associations::HasTable};
use rocket_contrib::databases::diesel::Queryable;
use rocket_contrib::databases::diesel::prelude::*;
use self::schema::users::dsl::*;

pub mod schema;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[derive(Queryable)]
pub struct User {
    pub email: String,
    pub hash: String,
    pub firstname: String,
    pub lastname: String,
    pub groups: String,
    pub pfp_link:String
}

#[get("/<email>")]
async fn get_user(email: String, db: DbConn) -> String
{
    let temp: User = User { email: String::from("dgramopadhye@gmail.com"), hash: String::from("123"), firstname:String::from("Dhruv"), lastname: String::from("Gramopadhye"), groups: String::from(""), pfp_link:String::from("https://dgramop.xyz") };
    dbg!(diesel::insert_into(schema::users::table).values(&temp));
    format!("{} ",email)
    //fetch user from mongodb
}

#[get("/")]
fn version() -> &'static str {
    "{\"status\":\"success\",\"api\":\"v1\"}"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/",routes![version])
        .launch()
        .await.expect("failed to launch rocket");
}
