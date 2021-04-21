use diesel::*;
use diesel_sample::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = db_connect();
    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}