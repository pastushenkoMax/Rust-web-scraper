#[macro_use] extern crate rocket;
extern crate scraper;
extern crate reqwest;
use rocket_dyn_templates::Template;
use rocket::fs::{relative, FileServer};
use serde_json::json;
mod service;


#[get("/")]
pub fn index() -> Template {
    let products = service::get_product();
    let context = json!({
        "products": products
    });
    Template::render("index", &context)

}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).attach(Template::fairing()).mount("/", FileServer::from(relative!("/static")))
}
