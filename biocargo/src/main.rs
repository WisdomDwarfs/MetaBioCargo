#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;

use rocket::http::ContentType;
// use rocket::request;
use rocket::Data;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::MultipartFormData;
use rocket_multipart_form_data::MultipartFormDataField;
use rocket_multipart_form_data::MultipartFormDataOptions;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {}
    let context = Context {};
    Template::render("index", context)
}

#[get("/translation")]
fn get_files() -> Template {
    #[derive(Serialize)]
    struct Context {}
    let context = Context {};
    Template::render("dashboard", context)
}

// #[derive(FromForm)]
// struct FileInput {
//     data_stream: String,
// }
#[post("/translation/form", data = "<input>")]
fn parse_form(content: &ContentType, input: Data) -> &'static str {
    let opts = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file")
            .content_type_by_string(Some(mime::TEXT))
            .unwrap(),
        MultipartFormDataField::raw("fingerprint").size_limit(50000 * 1024),
    ]);
    let mut mult_part_data = MultipartFormData::parse(content, input, opts).unwrap();
    let file_instance = mult_part_data.files.get("files");
    let file_fing = mult_part_data.raw.remove("fingerprint");

    if let Some(file_fields) = file_instance {
        let file_field = &file_fields[0];

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;
    } else if let Some(mut raw_fields) = file_fing {
        let raw_field = raw_fields.remove(0);

        let _content_type = raw_field.content_type;
        let _file_name = raw_field.file_name;
        let _raw = raw_field.raw;
    }
    "ok"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", routes![get_files])
        .register(catchers![not_found])
        .mount("/", routes![parse_form])
        .launch();
}
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
