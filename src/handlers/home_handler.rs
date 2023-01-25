use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};

use crate::model::blog_model::BlogBMC;
use crate::surrealdb_repo::SurrealDBRepo;

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>, db: Data<SurrealDBRepo>) -> impl Responder {
    let mut context = tera::Context::new();
    let mut blogs = match BlogBMC::get_all(db).await {
        Ok(b) => b,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "<h1>Internal Server Error!</h1><p>Error: {}</p>",
                    e
                ));
        }
    };

    blogs.sort_by(|a, b| b.get("order").cmp(&a.get("order")));

    context.insert("posts", &blogs);

    match templates.render("home.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
