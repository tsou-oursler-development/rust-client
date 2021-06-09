use crate::*;
use futures::prelude::*;
use irc::client::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConError {
    #[error("No command supplied")]
    ArgError(),
}

pub type ClientHandle = Arc<Mutex<Option<Client>>>;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

pub async fn create_client(
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    channel: &str,
) -> Client {
    let s = |s: &str| Some(s.to_owned());

    let config = Config {
        nickname: s(nick),
        server: s(srv),
        port: Some(port),
        use_tls: Some(use_tls),
        channels: vec![channel.to_string()],
        ..Config::default()
    };

    let client = tokio::task::block_in_place(|| Client::from_config(config));

    client.await.expect("create_client")
}

pub async fn start_receive(client: ClientHandle, my_channel: mpsc::Sender<Event>) {
    let mut stream = {
        let mut client_guard = client.lock().unwrap();
        let client_ref = client_guard.as_mut().unwrap();
        client_ref.identify().unwrap();
        client_ref.stream().unwrap()
    };
    let m1 = my_channel.clone();
    while let Some(m) = stream.next().await.transpose().unwrap() {
        let messager = m.to_string().clone();
        let _ = match m.command {
            Command::Response(_, _) => {
                let mut msg: Vec<_> = messager.split(' ').collect();
                m1.send(Event::IrcMessage(
                    msg.remove(0).to_string().replace(':', " "),
                    msg.join(" ").replace(':', " "),
                ))
                .unwrap();
            }
            Command::PRIVMSG(ref target, ref msg) =>
            // {
            {
                m1.send(Event::IrcMessage(target.to_string(), msg.to_string()))
                    .unwrap()
            }
            _ => m1
                .send(Event::IrcMessage("".to_string(), m.to_string()))
                .unwrap(),
        };
    }
}

pub fn send(client: &ClientHandle, message: &str) -> GenericResult<()> {
    let mut client = client.lock().unwrap();
    let client = client.as_mut().unwrap();
    let mut v: Vec<_> = message.split(' ').collect();
    let chan = if v.len() > 1 {
        if v[1].starts_with("#") {
            let check = v.remove(1);
            if check.is_channel_name() {
                check
            } else {
                ""
            }
        } else {
            ""
        }
    } else {
        ""
    };
    let res = match v[0] {
        "/PRIVMESSAGE" => client.send_privmsg(chan, v.drain(1..).collect::<Vec<_>>().join(" "))?,
        "/JOIN" => {
            if v.len() == 1 {
                client.send_join(chan)?
            } else {
                client.send_join(v.drain(1..).collect::<Vec<_>>().join(","))?
            }
        }
        "/INVITE" => client.send_invite(chan, v.remove(1))?,
        "/TOPIC" => client.send_topic(chan, v.remove(1))?,
        "/PART" => {
            if v.len() == 1 {
                client.send_part(chan)?
            } else {
                client.send_part(v.drain(1..).collect::<Vec<_>>().concat())?
            }
        }
        "/Quit" => client.send_quit(v.drain(1..).collect::<Vec<_>>().concat())?,
        _ => return Err(GenericError::from(ConError::ArgError())),
    };
    Ok(res)
}
