use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

 fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn establish_connection () -> DbPool {
  let database_url: String = env::var("DATABASE_URL")
  .expect("DATABASE_URL must be set in .env file");
let pool = init_pool(&database_url);

return pool
}