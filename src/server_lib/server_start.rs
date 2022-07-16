use std::net::SocketAddr;
use warp::{http::StatusCode, hyper::Method, Filter, Rejection, Reply};

pub async fn server_start(host: &SocketAddr) {
    println!("Starting Host At {}", host);

    let health_route = warp::path("health").and_then(health_handler);
    let log = warp::log("vtt_server");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ]);

    let routes = health_route.with(cors).with(log);

    warp::serve(routes).run(host.clone()).await;
}

async fn health_handler() -> std::result::Result<impl Reply, Rejection> {
    println!("Health Check");
    Ok(StatusCode::OK)
}
