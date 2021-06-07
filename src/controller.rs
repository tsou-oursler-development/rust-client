use crate::*;
use tokio_compat_02::FutureExt;
use futures::prelude::*;
use irc::client::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use thiserror::Error;
use async_std::task;
use tokio::*;

#[derive(Error, Debug)]
pub enum ConError {
    #[error("No command supplied")]
    ArgError(),
}

pub type ClientHandle = Arc<Mutex<Option<Client>>>;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

#[tokio::main]
pub async fn create_client(
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    channel: &str,
) ->  Client {
    eprintln!("connect::create_client() called");
    let s = |s: &str| Some(s.to_owned());
    let client: ClientHandle = Arc::new(Mutex::new(None));

    let config = Config {
        nickname: s(nick),
        server: s(srv),
        port: Some(port),
        use_tls: Some(use_tls),
        channels: vec![channel.to_string()],
        ..Config::default()
    };
    let client = tokio::spawn(async move {
        println!("connect::create_client() spawned");
        let temp_client = Client::from_config(config).await;
      
        //let mut set_client = client.lock().unwrap();
        //*set_client = Some(temp_client.unwrap());
        //drop(set_client);
    
        temp_client
        
    });
    
    let client = match client.await.unwrap(){
       Ok(t) => t,
       _ => panic!("create_client"),
    };

    client
    
}

    
pub fn start_receive(client: ClientHandle, event_channel: mpsc::Sender<Event>) {
    //task::block_in_place( async {run_stream(client, event_channel).compat().await } );
    tokio::task::block_in_place(|| { run_stream(client, event_channel) });
}

//#[tokio::main]
async fn run_stream(client: ClientHandle, my_channel: mpsc::Sender<Event>) {
    eprintln!("connect::run_stream() called");
    let mut client = client.lock().unwrap();
    let client = client.as_mut().unwrap();
    let mut stream = client.stream().unwrap();
    client.identify().unwrap();
    let m1 = my_channel.clone();
    while let Some(m) = stream.next().await.transpose().unwrap() {
        //rcv messages from server and send them to tui to print to screen
        eprintln!("{:?}", m);

        let _ = match m.command {
            Command::Response(Response::RPL_MOTD, _) => {
                m1.send(Event::IrcMotd(m.to_string())).unwrap()
            }
            Command::Response(Response::RPL_WELCOME, _) => {
                m1.send(Event::IrcWelcome).unwrap()
            }
            Command::Response(Response::RPL_NONE, _) => {
                m1.send(Event::IrcMessage(m.to_string())).unwrap()
            },
            _ => eprintln!("unknown message from IRC: {}", m.to_string()),
        };
    }
}

pub fn send(client: ClientHandle, message: &str) -> GenericResult<()> {
    let mut client = client.lock().unwrap();
    let client = client.as_mut().unwrap();
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
    let sender = client;
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
