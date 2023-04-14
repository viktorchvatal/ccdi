mod logger;

use logger::init_logger;
use warp::Filter;
use futures::StreamExt;
use futures::FutureExt;

const INDEX: &str = include_str!("static/index.html");

#[tokio::main]
async fn main() {
    init_logger();

    let websocket_service = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                ::log::info!("Websocket opened");
                let (tx, rx) = websocket.split();

                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let routes = warp::get().and(websocket_service.or(index));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;
}
