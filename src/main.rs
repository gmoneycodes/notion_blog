#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::response::content::Html;
use rocket_dyn_templates::{Template};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Context {
    title: String,
    articles: Vec<Article>,
}

#[derive(Serialize)]
struct Article {
    title: String,
    summary: String,
    image_url: String,
    read_more_url: String,
    published_date: String,
}

#[get("/")]
fn index() -> Html<String> {
    let context = Context {
        title: "Notion Blog".to_string(),
        articles: vec![
            Article {
                title: "Building a Notion Blog with Rust".to_string(),
                summary: "Dive into the details of creating a blog powered by Notion's API and Rust's robust system programming capabilities...".to_string(),
                image_url: "https://source.unsplash.com/random/800x600".to_string(),
                read_more_url: "#".to_string(),
                published_date: "Published on Jan 1, 2023".to_string(),
            },
            // Add more articles here
        ],
    };

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index", include_str!("templates/index.hbs"))
        .unwrap();

    let render = handlebars.render("index", &context).unwrap();
    Html(render)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

fn main() {
    rocket().launch()
}