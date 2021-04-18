#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
use serde_json::json;

use diesel::prelude::*;
use diesel::{SqliteConnection, Queryable};

pub mod schema;
use schema::users;

#[derive(Queryable,Insertable,FromForm)]
#[table_name="users"]
pub struct User {
    pub email: String,
    pub hash: String,
    pub firstname: String,
    pub lastname: String,
    pub groups: String,
    pub pfp_link:String
}

pub fn get_connection() -> SqliteConnection
{
    SqliteConnection::establish("/tmp/hack.sqlite").expect("Couldn't connect to the database!")
}

#[post("/users/<email>/register")]
fn put_user(email: String, user: rocket::request::Form<User>) -> String
{
//    use schema::users::dsl::*;
    let connection: SqliteConnection = get_connection();
    let temp: User = User { email: email.clone() , hash: String::from("123"), firstname:String::from("Dhruv"), lastname: String::from("Gramopadhye"), groups: String::from(""), pfp_link:String::from("https://dgramop.xyz") };
    let registration = diesel::insert_into(schema::users::table)
                .values(&temp)
                .execute(&connection);
    match registration {
        Ok(_) => {
            String::from("{\"status\":\"success\"}")
        },
        Err(e) => { 
            if e.to_string().contains("UNIQUE")
            {
                String::from("{\"status\":\"failure\",\"error\":\"already registered\"}")
            }
            else 
            {
                json!({
                    "status":"failure",
                    "error":e.to_string()
                }).to_string()
                //bad idea to leak the error, but this is just for a hackathon project
            }
        }
    }
    //fetch user from mongodb
}

#[get("/")]
fn version() -> &'static str {
    "{\"status\":\"success\",\"api\":\"v1\"}"
}

fn main() {
    rocket::ignite()
        .mount("/",routes![version,get_user])
        .launch();
}
