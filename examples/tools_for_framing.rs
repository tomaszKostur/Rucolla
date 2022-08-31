use bytes::{self, BytesMut};
use tokio::io::AsyncReadExt;

async fn read_to_bytes() {
    let mut some_b = BytesMut::with_capacity(32);

    let mut some_text = "totally random bytes".as_bytes();
    let n = some_text.read_buf(&mut some_b).await;

    println!("n: {:?}, {:?}", n, some_b);
}


struct Header {
    m_type: u8,
    len: u8
}


#[tokio::main]
async fn main() {
    read_to_bytes().await;
}
