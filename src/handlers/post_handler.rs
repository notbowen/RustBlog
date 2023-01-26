use std::{collections::BTreeMap, fmt::format};

use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use pulldown_cmark::{html, Options, Parser};
use surrealdb::sql::{thing, Value};

use crate::surrealdb_repo::SurrealDBRepo;

#[get("/posts/{post_name}")]
pub async fn post(
    tmpl: web::Data<tera::Tera>,
    db: Data<SurrealDBRepo>,
    post_id: web::Path<String>,
) -> impl Responder {
    let mut context = tera::Context::new();
    let options = Options::empty();

    let id = post_id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("<h1>Bad Request</h1><p>Invalid ID provided!</p>");
    }

    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

    match tmpl.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post :(</p>");
        }
    }
}
