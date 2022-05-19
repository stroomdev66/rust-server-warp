use warp::{http, Filter};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let receive_data = warp::post()
        .and(warp::path("receive_data"))
        .and(warp::path::end())
        .and_then(reply);

    let routes = receive_data.or(hello);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8090))
        .await;
}

// POST method
async fn reply() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Received data ok",
        http::StatusCode::OK,
    ))
}