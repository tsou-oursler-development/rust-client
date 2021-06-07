mod controller;
mod view;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    TuiMessage(String, String),
    TuiCredentials(String, String, String),
    TuiQuit,
    IrcMotd(String),
    IrcWelcome,
    IrcMessage(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (con_send, con_rcv) = mpsc::channel();
    let (mut tui, messages) = view::start_client(con_send.clone());
    
    let port = 6667;
    let use_tls = false;
    let ctlr = Arc::new(Mutex::new(None));
    
    let main_thread = thread::spawn(move || {
        loop {
            let message = con_rcv.recv().expect("receive channel closed");
            match message {
                Event::TuiMessage(name, message) => {
                    eprintln!("received message from {}: {}", name, message);
                    messages.append(format!("{}: {}\n", name, message));
                }
                Event::TuiQuit => {
                    eprintln!("quit");
                    // TODO: shut down client and tui.
                    break;
                }
                Event::TuiCredentials(_name, _channel, _server) => {
                    eprintln!("Check credentials");
                    let ctlr = Arc::clone(&ctlr);
                    let event_channel = con_send.clone();
                    let server = "irc.freenode.net";
                    let name = "Nottabot";
                    let channel = "botwar";
                    let _ = thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let client = rt.block_on(controller::create_client(
                            name,
                            server,
                            port,
                            use_tls,
                            channel,
                        ));
                        let mut rcvr = ctlr.lock().unwrap();
                        *rcvr = Some(client);
                        drop(rcvr);
                        controller::send(&ctlr, "/JOIN #botwar").unwrap();
                        rt.block_on(controller::start_receive(ctlr, event_channel))
                    });
                }
                _ => {
                    break;
                }
            }
        }
    });

    tui.run();
    main_thread.join().unwrap();
    Ok(()) 
}
