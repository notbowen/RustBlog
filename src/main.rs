use blog::start_blog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    start_blog("0.0.0.0:8080").await.unwrap().await?;
    Ok(())
}
