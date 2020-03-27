use std::convert::Infallible;

use diesel::RunQueryDsl;
use r2d2::PooledConnection;
use warp::Reply;
use warp::http::StatusCode;

use crate::db::Conn;
use crate::models::NewNgo;

pub async fn create_ngo(new_ngo: NewNgo, conn: PooledConnection<Conn>) -> Result<impl Reply, Infallible> {
    use crate::schema::ngos;

    let insertion = diesel::insert_into(ngos::table)
        .values(&new_ngo)
        .execute(&conn);

    match insertion {
        Ok(_) => {
            Ok(StatusCode::CREATED)
        }
        Err(_) => {
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}