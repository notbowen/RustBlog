use actix_files::Files;
use actix_web::{
    dev::Server,
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
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
                eprintln!("Parsing error(s): {}", e);
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

    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .app_data(db_data.clone())
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").use_last_modified(true))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::post)
            .service(handlers::create_post)
    })
    .bind(address)?
    .run();

    Ok(srv)
}
