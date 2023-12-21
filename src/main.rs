use std::os::fd::AsRawFd;

use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::io::Interest;



#[tokio::main]
async fn main() {

    let mut client_stream: Option<TcpStream> = None;

    tokio::spawn(async {

        let listener  = TcpListener::bind("0.0.0.0:8000").await.unwrap();

        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            client_stream = Some(stream);
            println!("Client connected from: {}:{}", addr.ip(), addr.port());

            println!("the stream fd is: {}", stream.as_raw_fd());
        }

    });
    
    let uds_listener = UnixListener::bind("/tmp/uds_test/uds_socket.sock").unwrap();

    loop {
        let (uds_stream, addr) = uds_listener.accept().await.unwrap();
        
        loop {
            let ready = uds_stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();

            
        
            
        }

    }


    
}
