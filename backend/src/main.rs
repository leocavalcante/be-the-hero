use be_the_hero::db::db_pool;
use be_the_hero::routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = db_pool(database_url);
    let app = routes::ngo_routes(pool);

    warp::serve(app)
        .run(([127, 0, 0, 1], 8888))
        .await;
}