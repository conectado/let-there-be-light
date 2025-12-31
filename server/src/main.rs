use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;

const LIGHT_PORT: u16 = 13355;
const USER_PORT: u16 = 8888;

#[derive(Clone)]
struct AppState(Arc<Mutex<Vec<String>>>);

#[tokio::main]
async fn main() {
    let light_listener = tokio::net::UdpSocket::bind(("0.0.0.0", LIGHT_PORT))
        .await
        .unwrap();

    let user_listener = tokio::net::TcpListener::bind(("0.0.0.0", USER_PORT))
        .await
        .unwrap();

    let data = Arc::new(Mutex::new(Vec::new()));
    let app_state = AppState(data.clone());

    let app = Router::new().route("/", get(root)).with_state(app_state);

    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 65553];
            let Ok(n) = light_listener.recv(&mut buf).await else {
                continue;
            };

            buf.truncate(n);

            let Ok(message) = String::from_utf8(buf) else {
                continue;
            };

            if data.lock().unwrap().len() < 100 {
                data.lock().unwrap().push(message);
            }
        }
    });

    axum::serve(user_listener, app).await.unwrap();
}

async fn root(State(state): State<AppState>) -> Html<String> {
    let mut response = Vec::new();
    response.extend(state.0.lock().unwrap().drain(..));
    let response = response.join("\n");
    Html(response)
}
