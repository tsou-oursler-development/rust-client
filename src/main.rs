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
    IrcMessage(String, String),
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
                Event::IrcMessage(name, message) => {
                    messages.append(format!("{}: {}\n", &name, &message));
                }
                Event::TuiMessage(name, message) => {
                    let send_message = format!("/PRIVMESSAGE {}", &message);
                    messages.append(format!("{}: {}\n", &name, &message));
                    controller::send(&ctlr, &send_message).unwrap();
                    // controller::send(&ctlr, &message).unwrap();
                }
                Event::TuiQuit => {
                    controller::send(&ctlr, "/Quit").unwrap();
                    //TODO close tui
                    break;
                }
                Event::TuiCredentials(name, channel, server) => {
                    let ctlr = Arc::clone(&ctlr);
                    let event_channel = con_send.clone();
                    messages.append(format!("{}: {}, {}\n", &name, &channel, &server));
                    let _ = thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let client = rt.block_on(controller::create_client(
                            &name, &server, port, use_tls, &channel,
                        ));
                        let mut rcvr = ctlr.lock().unwrap();
                        *rcvr = Some(client);
                        drop(rcvr);
                        let join_channel = format!("/JOIN {}", &channel);
                        controller::send(&ctlr, &join_channel).unwrap();
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
