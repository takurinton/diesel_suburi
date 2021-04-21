#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use diesel::result::Error;

use rocket_contrib::json::Json;

use diesel::*;
use diesel_sample::*;
use self::models::*;
use self::schema::posts::dsl::*;

#[get("/")]
fn index() -> Result<Json<Vec<Post>>, Error> {
    let conn = establish_connection();
    posts.
    load(&conn)
    .map(|post| Json(post))
}

#[get("/about")]
fn about() -> &'static str {
    "Hello, about!!!"
}
 
#[get("/takurinton")]
fn takurinton() -> &'static str {
    "Hello, takurinton!"
}

fn main() {
    rocket::ignite().mount("/", routes![
        index,
        about,
        takurinton
        ]).launch();
}