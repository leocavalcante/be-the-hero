use warp::Filter;

#[tokio::main]
async fn main() {
    let app = warp::get()
        .map(|| "Hello, World!");

    warp::serve(app)
        .run(([127, 0, 0, 1], 3333))
        .await;
}