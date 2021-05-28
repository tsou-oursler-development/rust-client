pub mod channel;
pub mod view;

use std::thread;

use view::tui::TuiMessage;

fn main() {
    let (mut tui, gui_channel) = view::tui::start_client();
    let worker = thread::spawn(move || {
        loop {
            let message = gui_channel.receive.recv().unwrap();
            match message {
                TuiMessage::Send(s) => {
                    println!("send: {}", s);
                }
                TuiMessage::Quit => {
                    println!("quit");
                    break;
                }
            }
        }
    });

    tui.run();

    worker.join().unwrap();
}
