#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;

use rocket::http::ContentType;
// use rocket::request;
use rocket::{Data, Request};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
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
#[post("/translation/form", data = "<input>")]
fn parse_form(content_type: &ContentType, input: Data) -> &'static str {
    let opts = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file")
            .content_type_by_string(Some(mime::APPLICATION_WWW_FORM_URLENCODED))
            .unwrap(),
        MultipartFormDataField::raw("fingerprint").size_limit(50000 * 1024),
    ]);
    // let plain_text: mime::Mime = "text/plain".parse().unwrap();
    let formdata = MultipartFormData::parse(content_type, input, opts);
    // let upload_file = value.files.get("files");
    // let file_fingerprint = value.raw.remove("fingerprint");
    println!("@param_value{:#?}:", formdata);
    // println!("@param_fingerprint {:#?}:", file_fingerprint);
    // if let Some(fields) = upload_file {
    //     let file_fields = &fields[0];
    //     let _content_type = &file_fields.content_type;
    //     let _filename = &file_fields.file_name;
    //     let _path = &file_fields.path;
    //     println!("@param_field: {:#?}", file_fields);
    // }
    // if let Some(mut raw_fingerprints) = file_fingerprint {
    //     let raw_data = &raw_fingerprints.remove(0);
    //     let _content_type = &raw_data.content_type;
    //     let _filename = &raw_data.file_name;
    //     let _raw = &raw_data.raw;
    //     println!("@param_raw: {:#?}", raw_data);
    // }
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
