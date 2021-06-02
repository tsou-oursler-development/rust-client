use std::sync::{Arc, Mutex};

use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, SelectView, TextContent,
    TextView,
};
use cursive::{Cursive, CursiveRunnable};

use crate::channel::Channel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiMessage {
    Message(String, String),
    Credentials(String, String),
    Quit,
}

type MChannel = Arc<Mutex<Channel<TuiMessage>>>;

fn check_credentials(
    s: &mut Cursive,
    messages: &TextContent,
    m: &MChannel,
    _server: &str,
    name: &str,
    _password: &str,
) {
    /*
    let m1 = Arc::clone(m);
    let m2 = Arc::clone(m);

    m1.lock()
        .unwrap()
        .send
        //Send server?
        .send(TuiMessage::Credentials(name.to_string(), password.to_string()))
        .unwrap();
    */

    //Receive credentials success or failure
    //If success:
    select_channel(s, &messages, &m, name);
}

fn select_channel(s: &mut Cursive, messages: &TextContent, mine: &MChannel, name: &str) {
    s.pop_layer();
    let name1 = name.to_string().clone();

    let channels = vec![
        "Channel 1",
        "Channel 2",
        "Channel 3",
        "Channel 4",
        "Channel 5",
    ];

    let channel_selection: Arc<Mutex<Option<usize>>> =
        Arc::new(Mutex::new(None));
    let cs = Arc::clone(&channel_selection);

    let mut channel_menu = SelectView::new()
        .on_submit(move |_, &item| {
            *cs.lock().unwrap() = Some(item);
        });
    for i in 0..channels.len() {
        channel_menu.add_item(channels[i], i);
    }

    let messages_clone = messages.clone();

    let m = Arc::clone(mine);
    let connect_button = Button::new("Connect", move |s| {
        let channel = *channel_selection.lock().unwrap();
        open_chat(s, &messages_clone, &m, &name1, &channels[channel.unwrap()])
    });

    let button_row = LinearLayout::horizontal()
        .child(connect_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("Quit", |s| s.quit()));

    let layout = LinearLayout::vertical()
        .child(channel_menu)
        .child(button_row);

    s.add_layer(Dialog::around(layout).title("Select Channel"));
}

fn open_chat(s: &mut Cursive, messages: &TextContent, m: &MChannel, name: &str, channel: &str) {
    s.pop_layer();
    let messages_clone = messages.clone();
    let name1 = name.to_string().clone();

    let chat_input = EditView::new().with_name("chat").fixed_width(24);

    let header = TextContent::new("Connected to ".to_string() + channel);

    let chat_wrapper = LinearLayout::horizontal()
        .child(TextView::new("Chat:"))
        .child(chat_input);

    let m1 = Arc::clone(m);
    let m2 = Arc::clone(m);
    let layout = LinearLayout::vertical()
        .child(TextView::new_with_content(header))
        .child(DummyView.fixed_height(1))
        .child(TextView::new_with_content(messages_clone))
        .child(chat_wrapper)
        .child(Button::new("Send", move |s| {
            //get message
            let message = s
                .call_on_name("chat", |view: &mut EditView| view.get_content())
                .unwrap();
            //clear input
            let _ = s
                .call_on_name("chat", |view: &mut EditView| view.set_content(""))
                .unwrap();
            //send message
            m1.lock()
                .unwrap()
                .send
                .send(TuiMessage::Message(name1.to_string(), message.to_string()))
                .unwrap();
        }))
        .child(Button::new("Quit", move |s| {
            m2.lock().unwrap().send.send(TuiMessage::Quit).unwrap();
            s.quit();
        }));
    s.add_layer(layout);
}

pub fn start_client() -> (CursiveRunnable, TextContent, Channel<TuiMessage>) {
    let mut siv = cursive::default();

    let messages = TextContent::new("");
    let messages_clone = messages.clone();

    let (mine, theirs) = Channel::pair();
    let mine = Arc::new(Mutex::new(mine));

    let _m = Arc::clone(&mine);

    let server_input = LinearLayout::horizontal()
        .child(TextView::new("Server:"))
        .child(EditView::new().with_name("server").fixed_width(22));

    let name_input = LinearLayout::horizontal()
        .child(TextView::new("Name:"))
        .child(EditView::new().with_name("name").fixed_width(24));

    let password_input = LinearLayout::horizontal()
        .child(TextView::new("Password:"))
        .child(EditView::new().with_name("password").fixed_width(20));

    let m = Arc::clone(&mine);
    let login_button = Button::new("Login", move |s| {
        let server = s
            .call_on_name("server", |view: &mut EditView| view.get_content())
            .unwrap();
        let name = s
            .call_on_name("name", |view: &mut EditView| view.get_content())
            .unwrap();
        let password = s
            .call_on_name("password", |view: &mut EditView| view.get_content())
            .unwrap();
        check_credentials(s, &messages, &m, &server, &name, &password)
    });

    let login_wrapper = OnEventView::new(
        LinearLayout::vertical()
            .child(server_input)
            .child(DummyView.fixed_height(1))
            .child(name_input)
            .child(password_input),
    );
    /*
    .on_event(event::Key::Enter, move |s| {
    let name = s
    .call_on_name("name", |view: &mut EditView| view.get_content())
    .unwrap();
    let password = s
    .call_on_name("password", |view: &mut EditView| view.get_content())
    .unwrap();
    check_credentials(s, &m, &name, &password)
    });
    */

    let m = Arc::clone(&mine);

    let button_row = LinearLayout::horizontal()
        .child(login_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("Quit", move |s| {
            m.lock().unwrap().send.send(TuiMessage::Quit).unwrap();
            s.quit();
        }));

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(DummyView.fixed_height(1))
                .child(login_wrapper)
                .child(DummyView.fixed_height(1))
                .child(button_row),
        )
        .title("Login"),
    );

    siv.add_global_callback('q', |s| s.quit());

    (siv, messages_clone, theirs)
}
