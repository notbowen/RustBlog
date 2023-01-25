use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use pulldown_cmark::{html, Options, Parser};

use crate::model::blog_model::{Blog, BlogBMC};
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
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("<h1>Bad Request!</h1><p>Invalid ID!</p>");
    }

    let blog: Blog = match BlogBMC::get(db, &id).await {
        Ok(b) => serde_json::from_str(&b.to_string()).expect("Able to parse JSON"),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "<h1>Internal Server Error!</h1><p>Error: {}</p>",
                    e.to_string()
                ))
        }
    };

    let parser = Parser::new_ext(&blog.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &blog);

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

#[post("/create_post")]
pub async fn create_post(db: Data<SurrealDBRepo>, blog: Json<Blog>) -> HttpResponse {
    let data = Blog {
        id: blog.id.to_owned(),
        title: blog.title.to_owned(),
        content: blog.content.to_owned(),
        posted: blog.posted.to_owned(),
        author: blog.author.to_owned(),
        estimated_reading_time: blog.estimated_reading_time.to_owned(),
        order: blog.order.to_owned(),
    };

    let blog_detail = BlogBMC::create(db, "blog", blog.id.clone(), data).await;

    match blog_detail {
        Ok(b) => HttpResponse::Ok().json(b),
        Err(e) => HttpResponse::InternalServerError().body(format!(
            "<h1>Internal Server Error!</h1><p>Error: {}</p>",
            e.to_string()
        )),
    }
}
