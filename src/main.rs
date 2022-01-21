#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).await.ok()
}

#[get("/about")]
async fn about() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("about.html")).await.ok()
}

#[get("/style/<file>")]
async fn styles(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/style/").join(file)).await.ok()
}

#[get("/image/<file>")]
async fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/images/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about, styles, images])
}
