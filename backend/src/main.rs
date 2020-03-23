use std::collections::HashMap;

use warp::Filter;

#[tokio::main]
async fn main() {
    let app = warp::get()
        .map(|| {
            let mut body = HashMap::new();
            body.insert("event", "OminiStack Week 11.0");
            body.insert("user", "Leo Cavalcante");
            warp::reply::json(&body)
        });

    warp::serve(app)
        .run(([127, 0, 0, 1], 3333))
        .await;
}