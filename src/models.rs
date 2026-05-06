use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
}

use super::schema::posts;
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub slug: Option<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdatePostHandler {
    pub title: Option<String>,
    pub body: Option<String>,
    pub slug: Option<String>,
}
