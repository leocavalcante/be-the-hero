use std::convert::Infallible;

use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::{Pool, PooledConnection};
use warp::Filter;

pub type Conn = ConnectionManager<SqliteConnection>;

pub fn with_db(pool: Pool<Conn>) -> impl Filter<Extract=(PooledConnection<Conn>, ), Error=Infallible> + Clone {
    warp::any().map(move || {
        pool.get().expect("Could not get connection from pool")
    })
}

pub fn db_pool(database_url: String) -> Pool<Conn> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("Could not create connection pool")
}