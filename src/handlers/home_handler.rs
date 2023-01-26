use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{model::post_model::PostBMC, surrealdb_repo::SurrealDBRepo};

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>, db: Data<SurrealDBRepo>) -> impl Responder {
    let mut context = tera::Context::new();

    let mut posts = match PostBMC::get_all(db).await {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("<h1>Internal Server Error</h1><p>Error: {}</p>", e))
        }
    };

    posts.sort_by(|a, b| b.order.cmp(&a.order));

    context.insert("posts", &posts);

    match templates.render("home.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprint!("{}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("<h1>Internal Server Error</h1><p>Error: {}</p>", e))
        }
    }
}
