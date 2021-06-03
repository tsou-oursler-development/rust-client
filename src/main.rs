pub mod channel;
mod controller;
pub mod view;

use crate::channel::Channel;
use crate::controller::connect;
use async_std::task;
use controller::connect::ConMessage;
use controller::connect::ConMessage;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use view::tui::TuiMessage;

/*
struct con {
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    channels: &[&str],
}
*/
type MChannel = Arc<Mutex<Channel<ConMessage>>>;

/*
struct con {
    nick: &str,
    srv: &str,
    port: u16,
    use_tls: bool,
    channels: &[&str],
}
*/

fn main() {
    let (mut tui, messages, gui_channel) = view::tui::start_client();
    let mut nick: String = "".to_string();
    let mut srv: String = "".to_string();
    let port = 6667;
    let use_tls = false;
    let mut my_channel: String = "".to_string();
    //let (con_channel_send, con_channel_receive) = Channel::<ConMessage>::pair();
    //let mut con_channel = &MChannel::new(Mutex::new(Channel::<ConMessage>));
    //let mut con_channel = Channel::<ConMessage>::pair();
    let (mut con_send, mut con_rcv) = Channel::<ConMessage>::pair();

    let tui_worker = thread::spawn(move || loop {
        let message = gui_channel.receive.recv().unwrap();
        match message {
            TuiMessage::Message(name, message) => {
                messages.append(name.to_string() + ": " + &message + "\n");
            }
            TuiMessage::Quit => {
                println!("quit");
            }
            TuiMessage::Credentials(name, channel, server) => {
                println!("Check credentials");
                nick = name;
                srv = server;
                my_channel = channel;
                let (client, sender, my_con_channel) = task::block_on(
                    controller::connect::start_client(&nick, &srv, port, use_tls, &my_channel),
                );
                //let receiver = Arc::new(Mutex::new(my_con_channel));
                //con_channel = &Arc::clone(&receiver);
                con_rcv = my_con_channel;
            }
        }
    });

    /*
        let (client, sender, connect_channel) = controller::connect::start_client();
        let connect_worker = thread::spawn(move || loop {
            let message = connect_channel.receive.recv().unwrap();
            match message {
                ConMessage::Message(message) => {
                    messages.append(message.to_string() + "\n");
                }
                ConMessage::Quit => {
                    println!("quit");
                    break;
                }
                _ => {
                    ();
                    break;
                }
            }
        });

    */

    tui.run();

    tui_worker.join().unwrap();
    // connect_worker.join().unwrap();
}
