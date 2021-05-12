use std::net::TcpStream;

pub fn con() -> () {
    if let Ok(stream) = TcpStream::connect("localhost:6667") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect...");
    }
}
