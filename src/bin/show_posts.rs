use diesel::*;
use diesel_sample::*;
use self::models::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");

    for post in results {
        println!("title: {}, body: {}\n", post.title, post.body);
    }
}