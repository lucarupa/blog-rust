use crate::models::{NewPost, NewPostHandler, Post, UpdatePost, UpdatePostHandler};
use crate::repository_post;
use crate::service_database::DbPool;
use actix_web::{get, post, put, web, HttpResponse, Responder};
#[utoipa::path(get, path = "/", responses((status = 200, description = "get posts", body = Vec<Post>)))]
#[get("/")]
async fn index(poll: web::Data<DbPool>) -> impl Responder {
    match repository_post::select_all(&poll) {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}
#[utoipa::path(get, path = "/{slug}", responses((status = 200, description = "get posts", body = Post)))]
#[get("/{slug}")]
async fn find_by_slug(poll: web::Data<DbPool>, slug: web::Path<String>) -> impl Responder {
    match repository_post::find_by_slug(&poll, slug.to_string()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}
#[utoipa::path(get, path = "/{id}", responses((status = 200, description = "get posts", body = Post)))]
#[get("/{id}")]
async fn find_by_id(poll: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    match repository_post::find_by_id(&poll, id.parse::<i32>().unwrap()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}
#[utoipa::path(
    post,
    path = "/",
    request_body = NewPostHandler,
    responses((status = 201, description = "Post creado", body = Post))
)]
#[post("/")]
async fn create_post(poll: web::Data<DbPool>, data: web::Json<NewPostHandler>) -> impl Responder {
    let slug = data.title.clone().replace(" ", "-").to_lowercase();
    let new_post = NewPost {
        title: data.title.as_str(),
        slug: slug.as_str(),
        body: data.body.as_str(),
    };
    match repository_post::create(&poll, new_post) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}

#[utoipa::path(
    put,
    path = "/{id_post}",
    request_body = UpdatePostHandler,
    responses((status = 201, description = "Post creado", body = Post))
)]
#[put("/{id_post}")]
async fn update_post(
    poll: web::Data<DbPool>,
    id_post: web::Path<String>,
    data: web::Json<UpdatePostHandler>,
) -> impl Responder {
    let update_post = UpdatePost {
        title: data.title.clone(),
        slug: data.slug.clone(),
        body: data.body.clone(),
    };
    match repository_post::update(&poll, id_post.parse::<i32>().unwrap(), update_post) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    }
}
