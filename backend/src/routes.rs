use r2d2::Pool;
use warp::{Filter, Rejection, Reply};

use crate::controllers;
use crate::db::{Conn, with_db};

pub fn create_ngo(pool: Pool<Conn>) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path!("ngos")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(controllers::create_ngo)
}