#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template};
use std::collections::HashMap;
use std::fs;
//use std::env;
//use serde::Serialize;

#[get("/")]
async fn index() -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("index", &context)
}

#[get("/about")]
async fn about() -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("about", &context)
}

#[get("/test_post")]
async fn test_post() -> Template {
    let post = fs::read_to_string("static/posts/ascii.html").
        expect("Could not open ascii post");

    let mut context = HashMap::<&str, &str>::new();
    context.insert("post", &post);
    Template::render("post", &context)
}

#[get("/style/<file>")]
async fn styles(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/style/").join(file)).await.ok()
}

#[get("/image/<file>")]
async fn images(file: PathBuf) -> Option<NamedFile> {
    println!("HI");
    NamedFile::open(Path::new("static/images/").join(file)).await.ok()
}

#[get("/image/posts/<post>/<file>")]
async fn post_images(post: PathBuf, file: PathBuf) -> Option<NamedFile> {
    println!("HI");
    NamedFile::open(Path::new("static/images/posts/").join(post).join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about, styles, images, post_images, test_post]).attach(Template::fairing())
}
