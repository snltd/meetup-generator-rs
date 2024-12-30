mod utils;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::path::PathBuf;
use std::sync::Arc;
use utils::types::{Agenda, Meetup, Talk};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(meetup: &State<Arc<Meetup>>) -> Template {
    let meetup = &**meetup;
    Template::render("index", context! { agenda: meetup.agenda(5) })
}

#[get("/api/agenda")]
fn api_agenda(meetup: &State<Arc<Meetup>>) -> Json<Agenda> {
    let meetup = &**meetup;
    Json(meetup.agenda(5))
}

#[get("/api/talk")]
fn api_talk(meetup: &State<Arc<Meetup>>) -> Json<Talk> {
    let meetup = &**meetup;
    Json(meetup.single_talk())
}

#[get("/api/<item>")]
fn api_item(meetup: &State<Arc<Meetup>>, item: &str) -> Option<Json<String>> {
    let meetup = &**meetup;
    let result = match item {
        "talker" => meetup.talker(),
        "refreshments" => meetup.refreshments(),
        "company" => meetup.company(),
        "role" => meetup.role(),
        "location" => meetup.location(),
        "date" => meetup.date(),
        _ => return None,
    };

    Some(Json(result))
}

#[launch]
fn rocket() -> _ {
    let src_dir = PathBuf::from("src/utils");
    let res_dir: PathBuf;
    let static_dir: &str;

    if src_dir.exists() {
        res_dir = src_dir;
        static_dir = "static";
    } else {
        res_dir = PathBuf::from("/");
        static_dir = "/static";
    };

    rocket::build()
        .manage(Arc::new(Meetup::new(
            &res_dir.join("all_the_things.toml"),
            &res_dir.join("words.gz"),
        )))
        .mount("/", routes![index, api_agenda, api_talk, api_item])
        .mount("/public", FileServer::from(static_dir))
        .attach(Template::fairing())
}
