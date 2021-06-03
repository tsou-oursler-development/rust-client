pub mod channel;
mod controller;
pub mod view;

use crate::controller::connect;
use std::thread;
use view::tui::TuiMessage;
use controller::connect::ConMessage;

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
    let mut nick: &str = "";
    let mut srv = "";
    let port = 6667;
    let use_tls = false;
    let mut channels = Vec::<&str>::new();

    let tui_worker = thread::spawn(move || loop {
        let message = gui_channel.receive.recv().unwrap();
        match message {
            TuiMessage::Message(name, message) => {
                messages.append(name.to_string() + ": " + &message + "\n");
            }
            TuiMessage::Quit => {
                println!("quit");
                break;
            }
            TuiMessage::Credentials(name, channel, server) => {
                //check_credentials(name, pass);
                nick = name;
                //channels.push(&channel);
                //srv = &server;

                println!("Check credentials");

                break;
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
