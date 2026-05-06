extern crate diesel;

use crate::models::NewPostHandler;
use crate::models::Post;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod repository_post;
mod routes;
mod schema;
mod service_database;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::index,
        routes::find_by_slug,
        routes::find_by_id,
        routes::create_post,
        routes::update_post,
    ),
    components(schemas(Post, NewPostHandler))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let conn = service_database::establish_connection();

    HttpServer::new(move || {
        App::new()
            .service(routes::index)
            .service(routes::find_by_slug)
            .service(routes::find_by_id)
            .service(routes::create_post)
            .service(routes::update_post)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(web::Data::new(conn.clone()))
    })
    .bind((host, port.parse().unwrap()))?
    .run()
    .await
}
