#![feature(proc_macro_hygiene, decl_macro)]
mod posts;
mod posts_service;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{serde::Serialize, State};

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
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![count])
        .manage(HitCount {
            count: AtomicUsize::new(0),
        })
        .attach(posts::stage())
}
