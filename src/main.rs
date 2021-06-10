//! IRC client main application driver.

mod controller;
mod view;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Enum for incoming and outgoing IRC messages. Messages from tui are `Tui*` and messages from Irc are `Irc*`.
///
///#Unimplemented: `IrcMotd` and `IrcWelcome` are not yet implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    TuiMessage(String, String),
    TuiCredentials(String, String, String),
    TuiQuit,
    IrcMotd(String),
    IrcWelcome,
    IrcMessage(String, String),
}

/// Main application driver. Initialize an mpsc::channel to receive messages from `controller` and `tui`. `tui` and `controller` receive an mpsc::Sender for sending to main.
///
/// Start the tui.
/// Create a main thread to loop for the life of the program overincoming messages from mpsc::channel.
///
/// Main listens for Event enums and then dispatches the appropriate code to complete actions.
///
/// `controller::create_client` and `controller::start_receive` are asynchronous functions, therefore they are called within a tokio runtime environment on their own anonymous thread.
/// `tui` functions run synchronously while some `controller` functions run asynchronously.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (con_send, con_rcv) = mpsc::channel();
    let (mut tui, messages) = view::start_client(con_send.clone());

    let port = 6667;
    let use_tls = false;
    let ctlr = Arc::new(Mutex::new(None));

    let main_thread = thread::spawn(move || loop {
        let message = con_rcv.recv().expect("receive channel closed");
        match message {
            Event::IrcMessage(name, message) => {
                messages.append(format!("{}: {}\n", &name, &message));
            }
            Event::TuiMessage(name, message) => {
                let mut send_message = format!("/PRIVMESSAGE {}", &message);
                messages.append(format!("{}: {}\n", &name, &message));

                if message.starts_with('/') {
                    send_message = message;
                } else if message.starts_with('@') {
                    let without_symbol = &message[1..message.len()];
                    send_message = format!("/PRIVMESSAGE {}", &without_symbol);
                }

                controller::send(&ctlr, &send_message).unwrap();

                if send_message == "/Quit" {
                    break;
                }
            }
            Event::TuiQuit => {
                messages.append("Quitting");
                controller::send(&ctlr, "/Quit").unwrap();
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
    });

    tui.run();
    main_thread.join().unwrap();
    tui.quit();
    Ok(())
}
