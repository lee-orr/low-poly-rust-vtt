use futures::{SinkExt, StreamExt, TryFutureExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    http::StatusCode,
    hyper::Method,
    ws::{Message, WebSocket},
    Filter, Rejection, Reply,
};

use crate::shared_lib::{
    chat::ChatMessage,
    protocol::{self, ProtocolMessage},
};

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<ProtocolMessage>>>>;

pub async fn server_start(host: &SocketAddr) {
    println!("Starting Host At {}", host);

    let health_route = warp::path("health").and_then(health_handler);
    let log = warp::log("vtt_server");

    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let ws_connection = warp::path("ws")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));

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

    let routes = health_route.or(ws_connection).with(cors).with(log);

    warp::serve(routes).run(host.clone()).await;
}

async fn user_connected(ws: WebSocket, users: Users) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    println!("User {} Connected", &my_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (channel_tx, channel_rx) = mpsc::unbounded_channel::<ProtocolMessage>();
    let mut channel_rx = UnboundedReceiverStream::new(channel_rx);

    tokio::task::spawn(async move {
        while let Some(message) = channel_rx.next().await {
            if let Ok(message) = serde_json::to_string(&message) {
                user_ws_tx
                    .send(Message::text(&message))
                    .unwrap_or_else(|e| {
                        eprintln!("websocket send error: {}", e);
                    })
                    .await;
            } else {
                eprintln!("Couldn't serialize message");
            }
        }
    });

    users.write().await.insert(my_id, channel_tx);

    while let Some(result) = user_ws_rx.next().await {
        if let Ok(msg) = result {
            if let Ok(msg) = msg.to_str() {
                if let Ok(msg) = serde_json::from_str::<ProtocolMessage>(&msg) {
                    println!("User Message: {:?}", msg);
                }
            }
        }
    }
}

async fn health_handler() -> std::result::Result<impl Reply, Rejection> {
    println!("Health Check");
    Ok(StatusCode::OK)
}
