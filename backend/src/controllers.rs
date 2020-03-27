use std::convert::Infallible;

use diesel::RunQueryDsl;
use r2d2::PooledConnection;
use serde::Serialize;
use warp::http::StatusCode;
use warp::Reply;
use warp::reply::{json, with_status};

use crate::db::Conn;
use crate::models::{NewNgo, Ngo};

pub async fn create_ngo(mut new_ngo: NewNgo, conn: PooledConnection<Conn>) -> Result<impl Reply, Infallible> {
    use crate::schema::ngos;

    let oid = Some(nanoid::nanoid!(8));
    new_ngo.oid = oid.clone();

    let insertion = diesel::insert_into(ngos::table)
        .values(&new_ngo)
        .execute(&conn);


    #[derive(Serialize)]
    struct Body {
        oid: Option<String>,
    }

    match insertion {
        Ok(_) => {
            let body = &Body { oid };
            Ok(with_status(json(body), StatusCode::CREATED))
        }
        Err(_) => {
            let body = &Body { oid: None };
            Ok(with_status(json(body), StatusCode::BAD_REQUEST))
        }
    }
}

pub async fn list_ngos(conn: PooledConnection<Conn>) -> Result<impl Reply, Infallible> {
    use crate::schema::ngos::dsl::*;

    #[derive(Serialize)]
    struct Body {
        status: bool,
        data: Option<Vec<Ngo>>,
    }

    match ngos.load::<Ngo>(&conn) {
        Ok(data) => {
            let body = Body { status: true, data: Some(data) };
            Ok(with_status(json(&body), StatusCode::OK))
        }
        Err(_) => {
            let body = Body { status: false, data: None };
            Ok(with_status(json(&body), StatusCode::BAD_REQUEST))
        }
    }
}