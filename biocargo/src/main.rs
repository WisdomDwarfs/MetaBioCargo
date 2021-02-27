#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {}
    let context = Context {};
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
