use rocket::{fairing::AdHoc, response::content::RawHtml};

use crate::posts_service::{Post, PostsService};
use rocket::serde::json::Json;

#[get("/<post_id>")]
async fn get_post(post_id: String) -> RawHtml<String> {
    let ps = PostsService::new().await;
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
async fn create_post(new_post: Json<Post>) {
    let ps = PostsService::new().await;
    ps.create_post(new_post.0).await;
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Managed Hit Count", |rocket| async {
        rocket.mount("/posts", routes![get_post, create_post])
    })
}
