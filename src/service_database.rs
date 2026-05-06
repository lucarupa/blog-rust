use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create connection pool")
}
