use actix_web::{get, web, HttpResponse, Responder};
use pulldown_cmark::{html, Options, Parser};
use std::{fs, io::Error};

use super::home_handler::Frontmatter;

fn extract_markdown(post_name: &str) -> Result<String, Error> {
    let markdown = match fs::read_to_string(format!("./posts/{}/post.md", post_name)) {
        Ok(markdown) => markdown,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(e);
        }
    };

    Ok(markdown)
}

fn extract_frontmatter(post_name: &str) -> Result<Frontmatter, Error> {
    let frontmatter_input =
        match fs::read_to_string(format!("./posts/{}/post_frontmatter.toml", post_name)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(e);
            }
        };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(e) => {
            eprint!("{:?}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Could not find post frontmatter",
            ));
        }
    };

    Ok(frontmatter)
}

#[get("/posts/{post_name}")]
pub async fn post(tmpl: web::Data<tera::Tera>, post_name: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();
    let options = Options::empty();

    let markdown_input = match extract_markdown(&post_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post :(</p>");
        }
    };

    let frontmatter = match extract_frontmatter(&post_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post :(</p>");
        }
    };

    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

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
