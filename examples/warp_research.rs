#![deny(warnings)]
use tokio::sync::mpsc;
use tokio::time;
use warp::Filter;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<usize>(10);
    let routes = warp::any()
        .and(warp::any().map(move || tx.clone()))
        .and_then(do_some);

    tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], 3030)));
    tokio::spawn(async move {
        loop {
            println!("RX loop: {:?}", rx.recv().await)
        }
    });
    time::sleep(time::Duration::from_secs(10)).await;
}

async fn do_some(
    sender: tokio::sync::mpsc::Sender<usize>,
) -> Result<impl warp::Reply, warp::Rejection> {
    sender.send(888).await.unwrap();
    Ok(warp::reply::with_status(
        "reply from do_some",
        warp::http::StatusCode::OK,
    ))
}
