use actix_files::Files;
use actix_web::dev::Server;
use actix_web::guard::fn_guard;
use actix_web::web;
use actix_web::{middleware, App, HttpResponse, HttpServer};
use surrealdb::Surreal;
use tera::Tera;

mod error;
mod handlers;
mod model;
mod prelude;
mod utils;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {e}");
                ::std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

static DB: Surreal<surrealdb::engine::local::Db> = Surreal::init();

pub async fn start_blog(address: &str) -> Result<Server, Box<dyn std::error::Error>> {
    log::info!("Starting blog server on {}", address);

    DB.connect::<surrealdb::engine::local::File>("/mnt/blog_data/blog.db")
        .await?;
    DB.use_ns("ns").use_db("db").await?;
    log::info!("Connected to database");

    let token = std::env::var("RUST_BLOG_AUTH").unwrap();

    let srv = HttpServer::new(move || {
        let token = token.clone();

        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(Files::new("/static", "static/").use_last_modified(true))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::post)
            .service(handlers::experience)
            .service(
                web::resource("/posts/{id}")
                    .guard(fn_guard(move |req| {
                        match req.head().headers().get("Authorization") {
                            Some(val) => val == token.clone().as_str(),
                            None => false,
                        }
                    }))
                    .route(web::post().to(handlers::create_post))
                    .route(web::put().to(handlers::update_post))
                    .route(web::delete().to(handlers::delete_post)),
            )
    })
    .bind(address)?
    .run();

    log::info!("Blog server started on {}", address);
    Ok(srv)
}
