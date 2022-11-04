use std::env::current_dir;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use kes::{config::get_config, logging::init_tracing, posts::PostManager, templates::Templates};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = web::Data::new(get_config());

    let port = config.port.clone();
    let workers = config.workers.clone();
    let posts_dir = config.posts_dir.clone();
    let log_level = config.log_level.clone();
    let log_format = config.log_format.clone();
    let mut asset_dir = current_dir().unwrap();
    asset_dir.push(&config.assets_dir);

    init_tracing(log_level, log_format);

    let templates = Templates::new(
        config.home_template.clone(),
        config.post_template.clone(),
        config.not_found_template.clone(),
    );
    let post_manager = web::Data::new(PostManager::new(posts_dir, &templates));
    let templates = web::Data::new(templates);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&config))
            .app_data(web::Data::clone(&post_manager))
            .app_data(web::Data::clone(&templates))
            .service(index)
            .service(not_found)
            .service(render_post)
            .service(actix_files::Files::new("/assets", asset_dir.clone()))
            .route(
                "/{wild:.*}",
                web::to(|| async {
                    HttpResponse::TemporaryRedirect()
                        .append_header(("location", "/404"))
                        .finish()
                }),
            )
    })
    .workers(workers)
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn index(
    templates: web::Data<Templates>,
    post_manager: web::Data<PostManager>,
) -> impl Responder {
    HttpResponse::Ok().body(templates.render_home(post_manager.get_post_list()))
}

#[get("/404")]
async fn not_found(templates: web::Data<Templates>) -> impl Responder {
    HttpResponse::Ok().body(templates.render_404())
}

#[get("/post/{post}")]
async fn render_post(
    post: web::Path<String>,
    post_manager: web::Data<PostManager>,
) -> impl Responder {
    tracing::info!("request to {post}");
    match post_manager.get(&post) {
        Some(content) => HttpResponse::Ok().body(content),
        None => HttpResponse::TemporaryRedirect()
            .append_header(("location", "/404"))
            .finish(),
    }
}
