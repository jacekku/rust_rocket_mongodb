use std::sync::Arc;

use rocket::{fairing::AdHoc, response::content::RawHtml, State};

use crate::posts_service::{ImplPostService, Post};
use rocket::serde::json::Json;

#[async_trait]
pub trait PostService {
    async fn get_post(&self, post_id: String) -> Option<Post>;
    async fn create_post(&self, post: Post);
}

struct Services {
    posts_service: Arc<dyn PostService + Send + Sync>,
}
#[get("/<post_id>")]
async fn get_post(post_id: String, services: &State<Services>) -> RawHtml<String> {
    let ps = &services.posts_service;
    let post = ps.get_post(post_id).await;
    match post {
        None => return RawHtml(format!("<h1>Not Found</h1>")),
        Some(post) => {
            return RawHtml(format!(
                "<h1>{}</h1><h2>{}</h2>",
                post.post_id, post.content
            ))
        }
    }
}

#[post("/", format = "json", data = "<new_post>")]
async fn create_post(new_post: Json<Post>, services: &State<Services>) {
    let ps = &services.posts_service;
    ps.create_post(new_post.0).await;
}

pub async fn stage() -> AdHoc {
    AdHoc::on_ignite("Managed Hit Count", |rocket| async {
        rocket
            .manage(Services {
                posts_service: Arc::from(ImplPostService::new().await),
            })
            .mount("/posts", routes![get_post, create_post])
    })
}
