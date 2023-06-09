use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use pulldown_cmark::{html, Options, Parser};
use tera::Tera;

use crate::{model::post_model::Post, DB};

#[get("/posts/{post_id}")]
pub async fn post(tmpl: Data<Tera>, post_id: Path<String>) -> impl Responder {
    let mut context = tera::Context::new();
    let options = Options::all();

    let id = post_id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("<h1>Bad Request</h1><p>Invalid ID!</p>");
    }

    let post_detail: Result<Post, surrealdb::Error> = DB.select(("post", id)).await;

    let post_detail = match post_detail {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::NotFound().body(
                "<h1>404 Not Found</h1><p>Couldn't find the post you were looking for :(</p>",
            );
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
            eprintln!("{e:?}");
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong while rendering post!</p>");
        }
    }
}

pub async fn create_post(_: Data<Tera>, new_post: Json<Post>) -> HttpResponse {
    let post_detail: Result<Post, surrealdb::Error> = DB
        .create(("post", &new_post.post_id))
        .content(Post {
            post_id: new_post.post_id.to_owned(),
            tags: new_post.tags.to_owned(),
            content: new_post.content.to_owned(),
            posted: new_post.posted.to_owned(),
            title: new_post.title.to_owned(),
            estimated_reading_time: new_post.estimated_reading_time.to_owned(),
            order: new_post.order.to_owned(),
        })
        .await;

    match post_detail {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_post(
    _: Data<Tera>,
    post_id: Path<String>,
    post_patch: Json<Post>,
) -> HttpResponse {
    let id = post_id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("<h1>Bad Request</h1><p>Invalid ID</p>");
    }

    let update_result: Result<Post, surrealdb::Error> = DB
        .update(("post", &id))
        .content(Post {
            post_id: post_patch.post_id.to_owned(),
            tags: post_patch.tags.to_owned(),
            content: post_patch.content.to_owned(),
            posted: post_patch.posted.to_owned(),
            title: post_patch.title.to_owned(),
            estimated_reading_time: post_patch.estimated_reading_time.to_owned(),
            order: post_patch.order.to_owned(),
        })
        .await;

    match update_result {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_post(_: Data<Tera>, post_id: Path<String>) -> HttpResponse {
    let id = post_id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("<h1>Bad Request</h1><p>Invalid ID</p>");
    }

    let delete_result: Result<Post, surrealdb::Error> = DB.delete(("post", &id)).await;

    match delete_result {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
