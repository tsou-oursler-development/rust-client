use crate::channel::Channel;
use futures::prelude::*;
use irc::client::prelude::*;
use std::sync::{Arc, Mutex};
pub async fn start_client(
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    channels: &[&str],
) -> Client {
    let s = |s: &str| Some(s.to_owned());
    let config = Config {
        nickname: s(nick),
        server: s(srv),
        port: Some(port),
        use_tls: Some(use_tls),
        channels: channels.into_iter().map(|s| s.to_string()).collect(),
        ..Config::default()
    };
    let mut client = Client::from_config(config).await.unwrap();
    run_stream(&mut client);
    client
}

#[tokio::main]
pub async fn run_stream(client: &mut Client) -> () {
    let mut stream = client.stream().unwrap();
    client.identify().unwrap();
    while let Some(m) = stream.next().await.transpose().unwrap() {
        //rcv messages from server and send them to tui to print to screen
        println!("{:?}", m);
    }
}

pub fn send_message() -> () {}
