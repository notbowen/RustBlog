use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();

    let mut frontmatters = match find_all_frontmatters() {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };

    frontmatters.sort_by(|a, b| b.order.cmp(&a.order));

    context.insert("posts", &frontmatters);

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
