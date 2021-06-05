use crate::channel::Channel;
use futures::prelude::*;
use irc::client::prelude::*;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConError {
    #[error("No command supplied")]
    ArgError(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConMessage {
    Message(String),
    Credentials(String),
    ChanList(Vec<String>),
    Ok,
    Quit,
}

type MChannel = Arc<Mutex<Channel<ConMessage>>>;
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
pub async fn start_client(
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    //channels: &[&str],
    channel: &str,
) -> (Client, Sender, Channel<ConMessage>) {
    println!("connect::start_client() called");
    let (mine, theirs) = Channel::pair();
    let mine = Arc::new(Mutex::new(mine));
    let m = Arc::clone(&mine);
    let s = |s: &str| Some(s.to_owned());

    let config = Config {
        nickname: s(nick),
        server: s(srv),
        port: Some(port),
        use_tls: Some(use_tls),
        //channels: channels.into_iter().map(|s| s.to_string()).collect(),
        channels: vec![channel.to_string()],
        ..Config::default()
    };
    let mut client = Client::from_config(config).await.unwrap();
    let sender = client.sender();
    //need a thread to run_stream and a thread to return client, sender
    //run_stream(&mut client, &m);
    (client, sender, theirs)
}

#[tokio::main]
pub async fn run_stream(client: &mut Client, my_channel: &MChannel) -> () {
    println!("connect::run_stream() called");
    let mut stream = client.stream().unwrap();
    client.identify().unwrap();
    let m1 = Arc::clone(my_channel);
    while let Some(m) = stream.next().await.transpose().unwrap() {
        //rcv messages from server and send them to tui to print to screen
        println!("{:?}", m);

        let _ = match m.command {
            Command::Response(Response::RPL_MOTD, _) => m1
                .lock()
                .unwrap()
                .send
                .send(ConMessage::Message(m.to_string()))
                .unwrap(),
            Command::Response(Response::RPL_WELCOME, _) => {
                m1.lock().unwrap().send.send(ConMessage::Ok).unwrap()
            }
            _ => (),
        };
    }
}

//fn into_args(index: usize, v: &mut Vec<&str>) -> &'a str {
//  &v.drain(index..).collect::<Vec<_>>().concat()
//}
pub fn receive(sender: &Sender, message: &str) -> GenericResult<()> {
    //irc::error::Result<()> {
    let mut v: Vec<_> = message.split(' ').collect();
    let chan = match &v[1].starts_with("#") {
        true => {
            let check = v.remove(1);
            if check.is_channel_name() {
                check
            } else {
                ""
            }
        }
        false => "",
    };
    let res = match v[0] {
        "/PRIVMESSAGE" => sender.send_privmsg(chan, v.drain(1..).collect::<Vec<_>>().concat())?,
        "/JOIN" => {
            if v.len() == 1 {
                sender.send_join(chan)?
            } else {
                sender.send_join(v.drain(1..).collect::<Vec<_>>().join(","))?
            }
        }
        "/INVITE" => sender.send_invite(chan, v.remove(1))?,
        "/TOPIC" => sender.send_topic(chan, v.remove(1))?,
        "/PART" => {
            if v.len() == 1 {
                sender.send_part(chan)?
            } else {
                sender.send_part(v.drain(1..).collect::<Vec<_>>().concat())?
            }
        }
        "/Quit" => sender.send_quit(v.drain(1..).collect::<Vec<_>>().concat())?,
        _ => return Err(GenericError::from(ConError::ArgError())),
    };
    Ok(res)
}
