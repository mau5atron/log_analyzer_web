#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use rocket_dyn_templates::{Template, context};

// #[get("/hello/world")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {field: "some_value"})
}

// #[launch] infers the return type
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
}