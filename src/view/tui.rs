use std::sync::{Arc, Mutex};

use cursive::event;
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, TextView, TextContent,
};
use cursive::{Cursive, CursiveRunnable};

use crate::channel::Channel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiMessage {
    Send(String),
    Quit,
}

type MChannel = Arc<Mutex<Channel<TuiMessage>>>;

pub fn login(s: &mut Cursive, mine: &MChannel) {
    s.pop_layer();

    let name_input = LinearLayout::horizontal()
        .child(TextView::new("Name:"))
        .child(EditView::new().with_name("name").fixed_width(24));

    let password_input = LinearLayout::horizontal()
        .child(TextView::new("Password:"))
        .child(EditView::new().with_name("password").fixed_width(20));

    //Using "on_submit" for either name_input or password_input
    //only extracts the name or password text, respectively.
    let m = Arc::clone(mine);
    let login_wrapper = OnEventView::new(
        LinearLayout::vertical()
            .child(name_input)
            .child(password_input),
    )
    .on_event(event::Key::Enter, move |s| {
        let name = s
            .call_on_name("name", |view: &mut EditView| view.get_content())
            .unwrap();
        let password = s
            .call_on_name("password", |view: &mut EditView| view.get_content())
            .unwrap();
        check_credentials(s, &m, &name, &password)
    });

    let m = Arc::clone(mine);
    let login_button = Button::new("login", move |s| {
        let name = s
            .call_on_name("name", |view: &mut EditView| view.get_content())
            .unwrap();
        let password = s
            .call_on_name("password", |view: &mut EditView| view.get_content())
            .unwrap();
        check_credentials(s, &m, &name, &password)
    });

    let m = Arc::clone(mine);
    let button_row = LinearLayout::horizontal()
        .child(login_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("[q]uit", move |s| {
            m.lock().unwrap().send.send(TuiMessage::Quit).unwrap();
            s.quit();
        }));

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(DummyView.fixed_height(1))
                .child(login_wrapper)
                .child(DummyView.fixed_height(1))
                .child(button_row),
        )
        .title("Login"),
    );
}

fn check_credentials(s: &mut Cursive, m: &MChannel, name: &str, password: &str) {
    s.pop_layer();

    let is_correct = verify(name, password);

    if is_correct {
        select_channel(s, m, name);
    } else {
        s.add_layer(Dialog::text("Incorrect username or password"));
    }
}

fn verify(_name: &str, _password: &str) -> bool {
    true
}

fn select_channel(s: &mut Cursive, mine: &MChannel, _name: &str) {
    s.pop_layer();

    //let name_copy = name.clone();

    let channel_input = LinearLayout::horizontal()
        .child(TextView::new("Channel name:"))
        .child(EditView::new().with_name("channel_name").fixed_width(24));

    let m = Arc::clone(mine);
    let connect_button = Button::new("Connect", move |s| {
        let channel = s
            .call_on_name("channel_name", |view: &mut EditView| view.get_content())
            .unwrap();
        //open_chat(s, &channel, &name)
        open_chat(s, &m, &channel)
    });

    let button_row = LinearLayout::horizontal()
        .child(connect_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("[q]uit", |s| s.quit()));

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(DummyView.fixed_height(1))
                .child(channel_input)
                .child(button_row),
        )
        .title("Connect to a channel"),
    );
}

//    fn open_chat(s: &mut Cursive, channel: &str, name: &str) {
fn open_chat(s: &mut Cursive, m: &MChannel, _channel: &str) {
    s.pop_layer();
    let message_sender = TextContent::new("");

    let chat_input = EditView::new().with_name("chat").fixed_width(24);

    let chat_wrapper = LinearLayout::horizontal()
        .child(TextView::new("Chat:"))
        .child(chat_input);


    let m1 = Arc::clone(m);
    let m2 = Arc::clone(m);
    let layout = LinearLayout::vertical()
        .child(TextView::new_with_content(message_sender))
        .child(chat_wrapper)
        .child(Button::new("Send", move |s| {
            let message = s
            .call_on_name("chat", |view: &mut EditView| view.get_content())
            .unwrap();
            let _ = s
            .call_on_name("chat", |view: &mut EditView| view.set_content(""))
            .unwrap();
            m1.lock().unwrap().send.send(TuiMessage::Send(message.to_string())).unwrap();
        }))
        .child(Button::new("Quit", move |s| {
            m2.lock().unwrap().send.send(TuiMessage::Quit).unwrap();
            s.quit();
        }));
    s.add_layer(layout);
}

pub fn start_client() -> (CursiveRunnable, Channel<TuiMessage>) {
    let mut siv = cursive::default();

    let (mine, theirs) = Channel::pair();
    let mine = Arc::new(Mutex::new(mine));

    let m = Arc::clone(&mine);
    let connect_button = Button::new("connect", move |s| {
        let _command = s
            .call_on_name("connect_input", |view: &mut EditView| view.get_content())
            .unwrap();
        login(s, &m);
    });

    let m = Arc::clone(&mine);
    let _button_row = LinearLayout::horizontal()
        .child(connect_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("[q]uit", move |s| {
            m.lock().unwrap().send.send(TuiMessage::Quit).unwrap();
            s.quit();
        }));

    #[cfg(any())]
    {
        siv.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(DummyView.fixed_height(1))
                    .child(TextView::new("Type '/connect' to connect to the server:"))
                    .child(
                        EditView::new()
                            .on_submit(connect_to_server)
                            .with_name("connect_input")
                            .fixed_width(28),
                    )
                    .child(DummyView.fixed_height(1))
                    .child(button_row),
            )
            .title("Welcome"),
        );
    }

    siv.add_global_callback('q', |s| s.quit());

    (siv, theirs)
}
