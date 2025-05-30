use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use mockall::mock;
use mockall::predicate::*;
use std::env;
use std::error::Error;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Re-export Pool for use in other modules
pub use diesel::r2d2::Pool;

/// Establish a connection to the database
pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool() {
        // Set test database URL for testing
        env::set_var("DATABASE_URL", "postgres://localhost/testbot_test");

        let pool = establish_connection();
        assert!(pool.get().is_ok());
    }
}
