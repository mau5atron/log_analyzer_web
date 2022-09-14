#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        title: "Home!"
    })
}

#[get("/hello/<name>")] 
pub fn get_name(name: &str) -> Template {
    Template::render("items", context! {
        title: "Hello!",
        name: Some(name),
        items: vec!["One", "Two", "Three"]
    })
}

// #[launch] infers the return type
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![get_name])
    .mount("/", FileServer::from(relative!("static")))
    .attach(Template::fairing())
}