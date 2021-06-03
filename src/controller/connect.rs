use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn con() -> TcpStream {
    let mut stream = TcpStream::connect("localhost:6667").expect("Couldn't connect...");
    let mut rbuf = [0; 2048];
    let message = irc::proto::message::Message::new(
        Some("boursler!boursler@localhost.me"),
        "JOIN",
        vec!["#channel"],
    )
    .unwrap();
    println!("Message: {}", message.to_string());
    stream.write(message.to_string().as_bytes()).unwrap();
    let res = stream.read(&mut rbuf[..]).unwrap();
    println!("Server response: {:?}", &rbuf[..res]);
    println!("In str form: {}", str::from_utf8(&rbuf).unwrap());

    stream
}
