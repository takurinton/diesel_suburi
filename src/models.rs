use diesel;
// use diesel::prelude::*;
// use super::schema::posts;

use diesel::{Queryable};
use serde::{Serialize};

#[derive(Serialize, Queryable)] 
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {}