use actix_web::{get, web, HttpResponse, Responder};

use crate::{model::post_model::Post, DB};

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let context = tera::Context::new();

    match templates.render("home.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprint!("{e}");
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("<h1>Internal Server Error</h1><p>Error: {e}</p>"))
        }
    }
}

#[get("/posts")]
pub async fn posts(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();

    let db_posts: Result<Vec<Post>, surrealdb::Error> = DB.select("post").await;

    let mut db_posts = match db_posts {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("<h1>Internal Server Error</h1><p>Error: {e}</p>"))
        }
    };

    dbg!(&db_posts);

    db_posts.sort_by(|a, b| b.order.cmp(&a.order));

    context.insert("posts", &db_posts);

    match templates.render("posts.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprint!("{e}");
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("<h1>Internal Server Error</h1><p>Error: {e}</p>"))
        }
    }
}
