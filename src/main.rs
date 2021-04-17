#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

mod user {
    use super::DbConn;

    #[derive(diesel::Queryable)]
    struct User {
        email: String,
        hash: Option<String>,
        firstname: String,
        lastname: String,
        groups: Vec<String>,
        pfp_link:String
    }

    #[get("/<email>")]
    async fn get_user(email: String, db: DbConn) -> String
    {
        
        //let dbs: Vec<_> = state.inner().list_database_names(None, None).await.expect("oops");
        //db.0
        format!("{} ",email)
        //fetch user from mongodb
    }

/*    #[post("/<email>")]
    fn update_user(email: String) -> String
    {
    }

    #[get("/<email>")]
    fn login(email: String) -> String
    {
    }

    #[get("/<email>")]
    fn register(email: String) -> String
    {
    }

    #[get("/<email>")]
    fn projects(email: String) -> String
    {
    }
    */
}

/*mod project {
    struct Project {
        members: Vec<String>,
        tasks: Vec<String>,
        emoji: String,
        id: String,
        name: String,
        description: String,
        owner: String
    }

    #[get("/<id>")]
    fn get_project(id: String) -> String
    {
    }

    #[post("/<id>")]
    fn update_project(id: String) -> String
    {
    }

    #[post("/<id>")]
    fn create_project(id: String) -> String
    {
    }
}
*/
#[get("/")]
fn version() -> &'static str {
    "{\"status\":\"success\",\"api\":\"v1\"}"
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    port: u16
}


#[rocket::main]
async fn main() {
    // Parse a connection string into an options struct.

     rocket::build()
        .mount("/", routes![version])
        .attach(AdHoc::config::<AppConfig>())
        .launch()
        .await
        .expect("failed to start server");
     println!("rocket done");

}
