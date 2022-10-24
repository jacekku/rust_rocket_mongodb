use rocket_db_pools::mongodb::{self, bson::doc, Collection};
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};

use crate::posts::PostService;

#[derive(Database)]
#[database("mongodb")]
struct Db(mongodb::Client);

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub post_id: String,
    pub content: String,
}

pub struct ImplPostService {
    posts_collection: Collection<Post>,
}

#[async_trait]
impl PostService for ImplPostService {
    async fn get_post(&self, post_id: String) -> Option<Post> {
        let cursor = self
            .posts_collection
            .find_one(doc! {"post_id": post_id}, None)
            .await;
        match cursor {
            Err(_) => return None,
            Ok(post) => return post,
        }
    }

    async fn create_post(&self, post: Post) {
        let _ = self.posts_collection.insert_one(post, None).await;
    }
}

impl ImplPostService {
    pub async fn new() -> Self {
        let test = std::env::var("TEST").expect("test to be gites");
        println!("{}", test);
        let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("1");

        let collection = client.database("rust_database").collection::<Post>("posts");
        ImplPostService {
            posts_collection: collection,
        }
    }
}
