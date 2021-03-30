use std::io::*;
use std::net::*;

fn main() {

    let listener = TcpListener::bind("0.0.0.0:27000").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // do something with the TcpStream
                //handle_connection(s);
                handle_connections(s);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                wait_for_fd();
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }

    let mut stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");

    let _written = stdout
        .write(message.as_bytes())
        .map_err(|err| println!("{:?}", err));

}

fn handle_connections(stream: TcpStream) {

}