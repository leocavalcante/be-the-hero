use r2d2::Pool;
use warp::{Filter, Rejection, Reply};

use crate::controllers;
use crate::db::{Conn, with_db};

pub fn ngo_routes(pool: Pool<Conn>) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    let create = warp::path!("ngos")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(controllers::create_ngo);

    let list = warp::path!("ngos")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(controllers::list_ngos);

    create.or(list)
}