use actix_files::Files;
use actix_web::dev::Server;
use actix_web::guard::fn_guard;
use actix_web::web::{self, Data};
use actix_web::{middleware, App, HttpResponse, HttpServer};
use surrealdb_repo::SurrealDBRepo;
use tera::Tera;

mod error;
mod handlers;
mod model;
mod prelude;
mod surrealdb_repo;
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

pub async fn start_blog(address: &str) -> Result<Server, std::io::Error> {
    let surreal = SurrealDBRepo::init()
        .await
        .expect("Able to connect to SurrealDB");
    let db_data = Data::new(surreal);

    let token = std::env::var("RUST_BLOG_AUTH").unwrap();

    let srv = HttpServer::new(move || {
        let token = token.clone();

        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .app_data(db_data.clone())
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(Files::new("/static", "static/").use_last_modified(true))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::posts)
            .service(handlers::post)
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

    Ok(srv)
}
