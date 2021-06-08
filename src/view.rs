use crate::*;

use std::{sync::mpsc, time, thread};

use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, Panel, ScrollView, TextContent,
    TextView, BoxView, NamedView,
};
use cursive::view::*;
use cursive::{Cursive, CursiveRunnable, Vec2};

fn check_credentials(
    s: &mut Cursive,
    messages: &TextContent,
    mine: &mpsc::Sender<Event>,
    server: &str,
    name: &str,
    irc_channel: &str,
) {
    s.pop_layer();

    let messages_clone = messages.clone();

    let layout = LinearLayout::vertical().child(TextView::new_with_content(messages_clone));
    
    let m = mine.clone();
    m.send(Event::TuiCredentials(
        name.to_owned(),
        irc_channel.to_owned(),
        server.to_owned(),
    ))
    .unwrap();
    let time = time::Duration::from_millis(1000);
    thread::sleep(time);
    open_chat(s, messages, m, name, irc_channel);
}

/*
fn select_channel(s: &mut Cursive, messages: &TextContent, mine: mpsc::Sender<Event>, name: &str) {
    s.pop_layer();
    let name1 = name.to_string().clone();

    let channels = vec![
        "#Channel 1",
        "Channel 2",
        "Channel 3",
        "Channel 4",
        "Channel 5",
    ];

    let channel_selection: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    let cs = Arc::clone(&channel_selection);

    let mut channel_menu = SelectView::new().on_submit(move |_, &item| {
        *cs.lock().unwrap() = Some(item);
    });
    for i in 0..channels.len() {
        channel_menu.add_item(channels[i], i);
    }

    let messages_clone = messages.clone();

    let connect_button = Button::new("Connect", move |s| {
        let channel = *channel_selection.lock().unwrap();
        open_chat(s, &messages_clone, mine, &name1, &channels[channel.unwrap()])
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
*/

pub fn open_chat(
    s: &mut Cursive,
    messages: &TextContent,
    m: mpsc::Sender<Event>,
    name: &str,
    channel: &str,
) {
    let messages_clone = messages.clone();
    let name1 = name.to_string().clone();

    //let chat_content =
      //          ScrollView::new(messaruges_clone).scroll_strategy(ScrollStrategy::StickToBottom);
 

    let chat_input = EditView::new().with_name("chat").min_width(80);

    let header = TextContent::new("Connected to ".to_string() + channel);

    let chat_wrapper = LinearLayout::horizontal()
        .child(TextView::new("Chat:"))
        .child(chat_input);

    let m1 = m.clone();
    let m2 = m.clone();
    
    let send_button = Button::new("Send", move |s| {
            //get message
            let message = s
                .call_on_name("chat", |view: &mut EditView| view.get_content())
                .unwrap();
            //clear input
            let _ = s
                .call_on_name("chat", |view: &mut EditView| view.set_content(""))
                .unwrap();
            //send message
            m1.send(Event::TuiMessage(name1.to_string(), message.to_string()))
                .unwrap();
        });

    let quit_button = Button::new("Quit", move |s| {
            m2.send(Event::TuiQuit).unwrap();
            s.quit()});

    let button_row = LinearLayout::horizontal()
        .child(send_button)
        .child(DummyView.fixed_width(2))
        .child(quit_button);

    let chat_layout = LinearLayout::vertical()
        .child(TextView::new_with_content(header))
        .child(DummyView.fixed_height(1))
        .child(TextView::new_with_content(messages_clone))        
        .scrollable().scroll_strategy(ScrollStrategy::StickToBottom);

    let layout = LinearLayout::vertical()
        .child(chat_layout)
        .child(chat_wrapper)
        .child(button_row);

    let chat_window = BoxView::with_max_height(80, layout);

    s.add_layer(Dialog::around(Panel::new(chat_window)));
}

pub fn start_client(mine: mpsc::Sender<Event>) -> (CursiveRunnable, TextContent) {
    let mut siv = cursive::default();

    let messages = TextContent::new("");
    let messages_clone = messages.clone();

    let server_input = LinearLayout::horizontal()
        .child(TextView::new("Server:"))
        .child(EditView::new().with_name("server").fixed_width(22));

    let name_input = LinearLayout::horizontal()
        .child(TextView::new("Name:"))
        .child(EditView::new().with_name("name").fixed_width(24));

    let irc_channel_input = LinearLayout::horizontal()
        .child(TextView::new("Channel:"))
        .child(EditView::new().with_name("irc_channel").fixed_width(20));

    let m = mine.clone();
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
        check_credentials(s, &messages, &m, &server, &name, &irc_channel)
    });

    let login_wrapper = OnEventView::new(
        LinearLayout::vertical()
            .child(server_input)
            .child(DummyView.fixed_height(1))
            .child(name_input)
            .child(irc_channel_input),
    );

    let m = mine.clone();
    let button_row = LinearLayout::horizontal()
        .child(login_button)
        .child(DummyView.fixed_width(2))
        .child(Button::new("Quit", move |s| {
            m.send(Event::TuiQuit).unwrap();
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

    (siv, messages_clone)
}
