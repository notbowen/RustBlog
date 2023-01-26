use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use pulldown_cmark::{html, Options, Parser};
use tera::Tera;

use crate::{
    model::post_model::{Post, PostBMC},
    surrealdb_repo::SurrealDBRepo,
};

#[get("/posts/{post_id}")]
pub async fn post(
    tmpl: Data<Tera>,
    db: Data<SurrealDBRepo>,
    post_id: web::Path<String>,
) -> impl Responder {
    let mut context = tera::Context::new();
    let options = Options::empty();

    let id = post_id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("<h1>Bad Request</h1><p>Invalid ID!</p>");
    }

    let post_detail = match PostBMC::get(db, &id).await {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("<h1>Internal Server Error</h1><p>Error: {}</p>", e));
        }
    };

    let parser = Parser::new_ext(&post_detail.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &post_detail);

    match tmpl.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong while rendering post!</p>");
        }
    }
}

#[post("/posts")]
pub async fn create_post(
    _: Data<Tera>,
    db: Data<SurrealDBRepo>,
    new_post: Json<Post>,
) -> HttpResponse {
    let new_post = Post {
        post_id: new_post.post_id.to_owned(),
        author: new_post.author.to_owned(),
        content: new_post.content.to_owned(),
        posted: new_post.posted.to_owned(),
        title: new_post.title.to_owned(),
        estimated_reading_time: new_post.estimated_reading_time.to_owned(),
        order: new_post.order.to_owned(),
    };

    let post_detail = PostBMC::create(db, new_post).await;

    match post_detail {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("<h1>Internal Server Error<h1><p>Error: {}</p>", e)),
    }
}
