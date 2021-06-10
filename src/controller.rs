//! IRC client code to handle connection to IRC server through use of rust irc
//! crate. Responsibilities include parsing and sending messages from the tui
//! along to the server connection and receiving messags back from the server
//! and parsing them for the tui.

use crate::*;
use futures::prelude::*;
use irc::client::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use thiserror::Error;

///Argument error when tui sends unrecognized or unsupported request.
#[derive(Error, Debug)]
pub enum ConError {
    #[error("No command supplied")]
    ArgError(),
}

///Locking reference counting pointer to client
pub type ClientHandle = Arc<Mutex<Option<Client>>>;

///Error type allows for multiple types of errors to be thrown
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

///Result type either nothing or an error.
type GenericResult<T> = Result<T, GenericError>;

///Create client connection to IRC server. Receive credentials from main and
/// establish a connection, then return the client. Async function so must be
/// called in an asynchronous block.

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

///Initiate incoming stream from server. Once stream is running, create a loop
/// that does not end until client connection to server is severed. The loop
/// processes each message received from the server and formats it for its final
/// printing on the tui. The name of the sender is separated from the irc
/// message struct and sent as a string, along with the message body. When the
/// sender is the server, the server name is specified. When sender is unknown,
/// as for example occurs after quit is sent, 'unk' is printed.

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
            Command::PRIVMSG(ref target, ref msg) => match m.source_nickname() {
                Some(inner) => m1
                    .send(Event::IrcMessage(inner.to_string(), msg.to_string()))
                    .unwrap(),
                None => m1
                    .send(Event::IrcMessage(target.to_string(), msg.to_string()))
                    .unwrap(),
            },
            _ => match m.response_target() {
                Some(inner) => m1
                    .send(Event::IrcMessage(inner.to_string(), messager.to_string()))
                    .unwrap(),
                None => m1
                    .send(Event::IrcMessage("unk".to_string(), messager.to_string()))
                    .unwrap(),
            },
        };
    }
}

///Parse outgoing messages. Determine the command as expressed by the end user
/// and dispatch to the appropriate outgoing send function as provided by the
/// rust irc library. Returns a GenericResult, which may be None or an error.
/// The error may be thrown by the rust irc call to send or as an ArgError if
/// unrecognized arguments are passed to `controller::send`.
///
/// # Errors
///
/// * `GenericError`
pub fn send(client: &ClientHandle, message: &str) -> GenericResult<()> {
    let mut client = client.lock().unwrap();
    let client = client.as_mut().unwrap();
    let mut v: Vec<_> = message.split(' ').collect();
    let chan = if v.len() > 1 {
        if v[1].starts_with('#') {
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
        "/PRIVMESSAGE" => {
            if !chan.contains('#') {
                client.send_privmsg(v.remove(1), v.drain(1..).collect::<Vec<_>>().join(" "))?
            } else {
                client.send_privmsg(chan, v.drain(1..).collect::<Vec<_>>().join(" "))?
            }
        }
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
