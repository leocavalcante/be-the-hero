use serde::{Deserialize, Serialize};

use super::schema::ngos;

#[derive(Queryable, Serialize)]
pub struct Ngo {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub whats_app: String,
    pub city: String,
    pub state: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "ngos"]
pub struct NewNgo {
    pub name: String,
    pub email: String,
    pub whats_app: String,
    pub city: String,
    pub state: String,
}