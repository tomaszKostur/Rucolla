use std::io;
use std::net::SocketAddr;
use tokio::io::Interest;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    println!("Rucolla board server !!!");
    // NOTE: to test the server use command
    // netcat 127.0.0.1 6677
    serve_forever().await;
}

async fn serve_forever() {
    let listener = TcpListener::bind("127.0.0.1:6677")
        .await
        .expect("Error while creating listener");

    loop {
        let (socket, addr) = listener.accept().await.expect("Cannot accept connection");
        println!("Accepted: {:?}", addr);
        tokio::spawn(async move {
            serve_participant(socket, addr).await;
        });
    }
}

async fn serve_participant(data: TcpStream, participant: SocketAddr) {
    loop {
        let ready = data
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await
            .expect("Error check if data ready");

        if ready.is_readable() {
            let mut buf = vec![0; 32];
            match data.try_read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    println!("Readed {:?} bytes from {:?}, {:?}", n, participant, buf);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // TODO: Read about this WouldBlock error
                    continue;
                }
                Err(e) => {
                    panic!("Read errors not implemented. The error: {:?}", e);
                }
            }
        }
    }
    println!("Participant: {:?} closed its connection", participant);
}
