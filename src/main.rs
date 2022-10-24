#![feature(proc_macro_hygiene, decl_macro)]
mod posts;
mod posts_service;
use dotenv::dotenv;
use rocket::{serde::Serialize, State};
use std::sync::atomic::{AtomicUsize, Ordering};

#[macro_use]
extern crate rocket;

struct HitCount {
    count: AtomicUsize,
}

#[derive(Clone, Serialize)]
struct Item {
    id: usize,
}

#[get("/count")]
async fn count(hit_count: &State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);

    format!("Number of visits: {}", current_count)
}
#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let test = std::env::var("TEST").expect("test to be gites");
    println!("{}", test);
    rocket::build()
        .mount("/", routes![count])
        .manage(HitCount {
            count: AtomicUsize::new(0),
        })
        .attach(posts::stage().await)
}
