
pub mod channel;
pub mod view;
mod controller;

use std::thread;
use crate::controller::connect;
use view::tui::TuiMessage;

fn main() {
    let (mut tui, messages, gui_channel) = view::tui::start_client();

    let worker = thread::spawn(move || loop {
        let message = gui_channel.receive.recv().unwrap();
        match message {
            TuiMessage::Message(name, message) => {
                messages.append(name.to_string() + ": " + &message + "\n");
            }
            TuiMessage::Quit => {
                println!("quit");
                break;
            }
            TuiMessage::Credentials(_name, _pass) => {
                //check_credentials(name, pass);
                println!("Check credentials");
                break;
            }
        }
    });

    tui.run();

    worker.join().unwrap();
}
