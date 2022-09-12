use blackboard_lib::blackboard::{new_note, StickyNote};
use warp::Filter;
use tokio::sync::mpsc;

async fn serve_the_board(tx: mpsc::Sender<String>) {
    let rest_api = warp::path("note")
        .and(warp::path::param())
        .map(|param: String| {
            let send_out = tx.send(param);
            match send_out {
                Ok(()) => {
                    println!("sending done properly");
                },
                Err(e) => {
                    println!("some err when sending to update blackboard");
                }
            }
            "The retunr from rest_api"
        });

    warp::serve(rest_api).run(([127, 0, 0, 1], 6677)).await;
    println!("huehue")
}

async fn board_keeper(rx: mpsc::Receiver<String>) {
    let mut blackboard = Vec::<StickyNote>::new();
    loop {
        match rx.await {
            Ok(message) => {
                println!("blackboard updated by Sticky Note {:?}", message);
                blackboard.push(new_note());
            },
            Err(e) => {
                println!("Error while receiving message: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<StickyNote>();
    tokio::spawn(async move {serve_the_board(tx).await});
    tokio::spawn(async move {board_keeper(rx).await});
}
