//! IRC client text-based user interface. Provides the user with a textual display of a login menu
//! and all incoming and outgoing IRC messages and commands.

use crate::*;
use cursive::traits::*;
use cursive::view::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, Panel, ResizedView,
    TextContent, TextView,
};
use cursive::{Cursive, CursiveRunnable};
use std::{sync::mpsc, thread, time};
use thiserror::Error;

/// TUI input error.
#[derive(Error, Debug)]
pub enum TuiError {
    #[error("No command supplied")]
    ChannelError(),
}

/// Result type for TUI errors.
pub type Result<T> = std::result::Result<T, TuiError>;

/// Check that user has correctly formatted the channel name.
/// Send credentials through a channel to be verified.
///
///
/// # Arguments:
///
/// * `s`: A cursive object for running the client.
/// * `message_display`: A TextContent view to display irc messages.
/// * `event_sender`: an mpsc::Sender that sends tui events to the thread running the tui.
/// * `sever`: An &str with the name of the server to connect to.
/// * `name`: An &str of the user's nickname.
/// * `irc_channel`: An &str with the name of the channel to connect to.
///
///
/// # Return value:
///
/// * Result<(), TuiError>, returns TuiError::ChannelError() if the channel does not being with
/// `#`.
///
///
/// # Errors
///
/// * `TuiError::ChannelError` if `irc_channel` does not begin with `#`.
pub fn check_credentials(
    s: &mut Cursive,
    message_display: &TextContent,
    event_sender: &mpsc::Sender<Event>,
    server: &str,
    name: &str,
    irc_channel: &str,
) -> Result<()> {
    s.pop_layer();

    if !irc_channel.starts_with('#') {
        return Err(TuiError::ChannelError());
    }

    let sender_clone = event_sender.clone();
    sender_clone.send(Event::TuiCredentials(
        name.to_owned(),
        irc_channel.to_owned(),
        server.to_owned(),
    ))
    .unwrap();
    
    let time = time::Duration::from_millis(1000);
    thread::sleep(time);
    
    open_chat(s, message_display, sender_clone, name, irc_channel);

    Ok(())
}

/// Open a chat window and a text editor.
/// All incoming and outgoing messages are displayed in the window,
/// and a user can type their messages into the text editor.
/// Messages written to the tui are sent through a channel to be
/// received by the main thread and the irc protocol API.
/// The main thread appends the messages onto the tui window
/// `chat_window` created by open_chat.
///
///
/// # Arguments:
///
/// * `s`: A cursive object for running the client.
/// * `message_display`: A TextContent view to display irc messages.
/// * `event_sender`: an mpsc::Sender that sends tui events to the thread running the tui.
/// * `name`: An &str of the user's nickname.
/// * `irc_channel`: An &str with the name of the channel to connect to.
pub fn open_chat(
    s: &mut Cursive,
    message_display: &TextContent,
    event_sender: mpsc::Sender<Event>,
    name: &str,
    irc_channel: &str,
) {
    let message_clone = message_display.clone();
    let name_clone = name.to_string();

    let header = TextContent::new(format!("Connected to {}.\n\nType '#channel_name \
                                          [message]' to send a message to the channel.\nType 'username \
                                          [message]' or '@username [message]' to send a message to a user\n", irc_channel));

    let chat_input = EditView::new().with_name("chat").min_width(80);
    let chat_input_wrapper = LinearLayout::horizontal()
        .child(TextView::new("Chat:"))
        .child(chat_input);

    let sender_clone = event_sender.clone();
    let send_button = Button::new("Send", move |s| {
        let message = s
            .call_on_name("chat", |view: &mut EditView| view.get_content())
            .unwrap();
        let _ = s
            .call_on_name("chat", |view: &mut EditView| view.set_content(""))
            .unwrap();
        let words: Vec<String> = message.split(' ').map(|s| s.to_string()).collect();
        if words.len() == 1 {
            let upper_word = words[0].to_uppercase();
            match upper_word.as_str() {
                "QUIT" => {
                    event_sender.send(Event::TuiQuit).unwrap();
                }
                _ => {
                    event_sender.send(Event::TuiMessage(name_clone.clone(), message.to_string()))
                        .unwrap();
                }
            };
        } else {
            event_sender.send(Event::TuiMessage(name_clone.clone(), message.to_string()))
                .unwrap();
        }
    });

    let quit_button = Button::new("Quit", move |s| {
        sender_clone.send(Event::TuiQuit).unwrap();
        s.quit()
    });

    let button_row = LinearLayout::horizontal()
        .child(send_button)
        .child(DummyView.fixed_width(2))
        .child(quit_button);

    let chat_layout = LinearLayout::vertical()
        .child(TextView::new_with_content(message_clone))
        .child(chat_input_wrapper)
        .scrollable()
        .scroll_strategy(ScrollStrategy::StickToBottom);

    let window_layout = LinearLayout::vertical()
        .child(TextView::new_with_content(header))
        .child(DummyView.fixed_height(1))
        .child(chat_layout)
        .child(button_row);

    let chat_window = ResizedView::with_max_height(50, window_layout);

    s.add_layer(Dialog::around(Panel::new(chat_window)));

    s.set_fps(1);
}

/// Start the irc client.
/// Prompt a user to enter a server, name, and channel.
/// Display an error dialog box if the user does does enter a proper
/// channel name.
///
///
/// # Arguments:
///
/// * `event_sender`: an mpsc::Sender that sends tui events to the thread running the tui.
/// 
///
/// # Return values:
/// 
/// * A CursiveRunnable object to start the tui.
/// * A TextContent view to display irc messages.
pub fn start_client(event_sender: mpsc::Sender<Event>) -> (CursiveRunnable, TextContent) {
    let mut siv = cursive::default();

    let message_display = TextContent::new("");
    let message_clone = message_display.clone();

    let server_input = LinearLayout::horizontal()
        .child(TextView::new("Server:"))
        .child(EditView::new().with_name("server").fixed_width(22));

    let name_input = LinearLayout::horizontal()
        .child(TextView::new("Name:"))
        .child(EditView::new().with_name("name").fixed_width(24));

    let irc_channel_input = LinearLayout::horizontal()
        .child(TextView::new("Channel:"))
        .child(EditView::new().with_name("irc_channel").fixed_width(20));

    let sender_clone = event_sender.clone();
    let login_button = Button::new("Login", move |s| {
        let server = s
            .call_on_name("server", |view: &mut EditView| view.get_content())
            .unwrap();
        let name = s
            .call_on_name("name", |view: &mut EditView| view.get_content())
            .unwrap();
        let irc_channel = s
            .call_on_name("irc_channel", |view: &mut EditView| view.get_content())
            .unwrap();
        match check_credentials(s, &message_display, &sender_clone, &server, &name, &irc_channel) {
            Ok(()) => (),
            Err(e) => {
                s.pop_layer();
                let error_layout = LinearLayout::vertical()
                    .child(TextView::new(format!(
                        "Error: {:?}. Channel must begin with '#'.",
                        e
                    )))
                    .child(Button::new("Quit", move |s| {
                        s.quit();
                    }));

                s.add_layer(error_layout);
            }
        };
    });

    let login_wrapper = OnEventView::new(
        LinearLayout::vertical()
            .child(server_input)
            .child(DummyView.fixed_height(1))
            .child(name_input)
            .child(irc_channel_input),
    );

    let button_row = LinearLayout::horizontal()
        .child(login_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("Quit", move |s| {
            event_sender.send(Event::TuiQuit).unwrap();
            s.quit();
        }));

    let layout = Dialog::around(
        LinearLayout::vertical()
            .child(DummyView.fixed_height(1))
            .child(login_wrapper)
            .child(DummyView.fixed_height(1))
            .child(button_row),
    )
    .title("Login");

    siv.add_layer(layout);

    siv.add_global_callback('q', |s| s.quit());

    (siv, message_clone)
}
