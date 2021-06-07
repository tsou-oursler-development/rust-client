mod controller;
mod view;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use async_std::task;

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
    //let (mut tui, messages) = view::start_client(con_send.clone());
    
    //let port = 6667;
    //let use_tls = false;
    let ctlr = Arc::new(Mutex::new(None));
    
    /*
    let main_thread = thread::spawn(move || {
        loop {
            let message = con_rcv.recv().expect("receive channel closed");
            match message {
                Event::TuiMessage(name, message) => {
                    eprintln!("received message from {}: {}", name, message);
                    messages.append(format!("{}: {}\n", name, message));
                    //let ctlr = Arc::clone(&ctlr);
                    //controller::send(ctlr, &message);
                }
                Event::TuiQuit => {
                    eprintln!("quit");
                    // TODO: shut down client and tui.
                    break;
                }
                Event::TuiCredentials(name, channel, server) => {
                    eprintln!("Check credentials");
                    /*
                    messages.append(format!(
                        "NAME: {} CHANNEL: {} SERVER: {}",
                        name,
                        channel,
                        server,
                    ));
                    */
                    let ctlr = Arc::clone(&ctlr);
                    let event_channel = con_send.clone();
                    let _ = thread::spawn(move || {
                        let client = controller::create_client(
                            &name,
                            &server,
                            port,
                            use_tls,
                            &channel,
                        );
                        let mut rcvr = ctlr.lock().unwrap();
                        *rcvr = Some(client);
                        drop(rcvr);
                        //let client = client.identify();
                        //messages.append(format!("{:?}", &client));
                        controller::start_receive(ctlr, event_channel)
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
*/
    println!("before thread");
    let ctlr = Arc::clone(&ctlr);
    let event_channel = con_send.clone();
    let my_thread = thread::spawn(move || {
        println!("inside thread");
        let client = controller::create_client(
           "lily",
           "localhost.me",
            6667,
            false,
            "#channel",
        );
        let mut rcvr = ctlr.lock().unwrap();
        *rcvr = Some(client);
        drop(rcvr);
        controller::start_receive(ctlr, event_channel)
    });

    my_thread.join();
    Ok(())
}
