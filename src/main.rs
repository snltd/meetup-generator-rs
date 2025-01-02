mod utils;
use rocket::fs::FileServer;
use rocket::response::Redirect;
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

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    let mut root = PathBuf::from(rocket::fs::relative!("."));

    if !root.join("src").exists() {
        root = PathBuf::from("/");
    };

    let res_dir = root.join("resources");
    let static_dir = root.join("static");

    let meetup_generator = match Meetup::new(
        &res_dir.join("all_the_things.toml"),
        &res_dir.join("words.gz"),
    ) {
        Ok(mg) => mg,
        Err(e) => panic!("ERROR: Cannot instantiate meetup-generator: {}", e),
    };

    rocket::build()
        .manage(Arc::new(meetup_generator))
        .mount("/", routes![index, api_agenda, api_talk, api_item])
        .mount("/public", FileServer::from(static_dir))
        .register("/", rocket::catchers![not_found])
        .attach(Template::fairing())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use scraper::{Html, Selector};
    use std::collections::HashSet;

    #[test]
    fn test_renders_page() {
        let client = Client::tracked(rocket()).expect("invalid instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::HTML);
        assert!(response.into_string().unwrap().contains("#Devops Meetup"));
    }

    #[test]
    fn test_404_redirects_to_main_page() {
        let client = Client::tracked(rocket()).expect("invalid instance");
        let response = client.get("/some/nonsense/path").dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some("/"));
    }

    #[test]
    fn test_no_repeat_titles() {
        let client = Client::tracked(rocket()).expect("invalid instance");

        for _ in 0..400 {
            let response = client.get(uri!(super::index)).dispatch();
            let document = Html::parse_document(&response.into_string().unwrap());

            let titles = Selector::parse("span[class=\"ttitle\"]").unwrap();
            let mut seen = HashSet::new();

            for element in document.select(&titles) {
                let value = element.inner_html();
                if seen.contains(&value) {
                    panic!("Saw title twice");
                } else {
                    seen.insert(value);
                }
            }
        }
    }

    #[test]
    fn test_api() {
        let client = Client::tracked(rocket()).expect("invalid instance");

        let agenda_response = client.get(uri!(super::api_agenda)).dispatch();
        assert_eq!(agenda_response.status(), Status::Ok);

        let talk_response = client.get(uri!(super::api_talk)).dispatch();
        assert_eq!(talk_response.status(), Status::Ok);

        let items = vec![
            "company",
            "date",
            "location",
            "refreshments",
            "role",
            "talker",
        ];

        for item in items {
            let item_response = client.get(uri!(super::api_item(item))).dispatch();
            assert_eq!(
                item_response.status(),
                Status::Ok,
                "failed on API item {}",
                item
            );
            assert_eq!(item_response.content_type().unwrap(), ContentType::JSON);
        }
    }
}
