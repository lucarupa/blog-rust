use diesel::prelude::*;

use crate::models::{NewPost, Post, UpdatePost};
use crate::schema::posts::dsl::posts;
use crate::schema::posts::{id, slug};
use crate::service_database::DbPool;

pub fn select_all(poll: &DbPool) -> Result<Vec<Post>, diesel::result::Error> {
    let conn = &mut poll.get().expect("couldn't get connection from pool");
    posts.select(Post::as_select()).load(conn)
}

pub fn find_by_id(poll: &DbPool, post_id: i32) -> Result<Post, diesel::result::Error> {
    let conn = &mut poll.get().expect("couldn't get connection from pool");
    posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first(conn)
}

pub fn create(poll: &DbPool, post: NewPost) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts;
    let conn = &mut poll.get().expect("couldn't get connection from pool");
    diesel::insert_into(posts::table)
        .values(&post)
        .returning(Post::as_returning())
        .get_result(conn)
}

pub fn find_by_slug(poll: &DbPool, slug_find: String) -> Result<Post, diesel::result::Error> {
    let conn = &mut poll.get().expect("couldn't get connection from pool");
    posts
        .filter(slug.eq(slug_find.trim()))
        .select(Post::as_select())
        .first(conn)
}

pub fn update(
    poll: &DbPool,
    id_post: i32,
    post_update: UpdatePost,
) -> Result<Post, diesel::result::Error> {
    let conn = &mut poll.get().expect("couldn't get connection from pool");
    diesel::update(posts.find(id_post))
        .set(&post_update)
        .returning(Post::as_returning())
        .get_result(conn)
}
