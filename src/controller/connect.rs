use futures::prelude::*;
use irc::client::prelude::*;
#[tokio::main]
pub async fn con(nick: &str, srv: &str, port: u16, use_tls: bool, channels: &[&str]) -> () {
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
    let mut stream = client.stream().unwrap();
    client.identify().unwrap();
    while let Some(m) = stream.next().await.transpose().unwrap() {
        println!("{:?}", m);
    }
}

pub fn send_message() -> () {}
