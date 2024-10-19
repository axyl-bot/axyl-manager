use serde_json::Value;
use warp::Filter;

pub async fn run_test_server() {
    let api = warp::post()
        .and(warp::path("api"))
        .and(warp::path("user"))
        .and(warp::body::json())
        .map(|user_info: Value| {
            println!("Received user info: {:?}", user_info);
            warp::reply::json(&"User info received")
        });

    println!("Test server running on http://localhost:3000");
    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
