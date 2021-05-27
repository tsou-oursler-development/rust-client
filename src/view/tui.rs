pub mod tui {
    use std::thread;
    use std::sync::mpsc;
    use cursive::event;
    use cursive::traits::*;
    use cursive::views::{
        Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, TextView, TextContent,
    };
    use cursive::Cursive;

    fn connect_to_server(s: &mut Cursive, command: &str) {
        s.pop_layer();

        match command {
            "/connect" => {
                login(s);
            }
            _ => {
                s.add_layer(
                    Dialog::text(format!("Command {} not recognized", command))
                        .button("[q]uit", |s| s.quit()),
                );
            }
        }
    }

    fn login(s: &mut Cursive) {
        s.pop_layer();

        let name_input = LinearLayout::horizontal()
            .child(TextView::new("Name:"))
            .child(EditView::new().with_name("name").fixed_width(24));

        let password_input = LinearLayout::horizontal()
            .child(TextView::new("Password:"))
            .child(EditView::new().with_name("password").fixed_width(20));

        //Using "on_submit" for either name_input or password_input
        //only extracts the name or password text, respectively.
        let login_wrapper = OnEventView::new(
            LinearLayout::vertical()
                .child(name_input)
                .child(password_input),
        )
        .on_event(event::Key::Enter, |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            let password = s
                .call_on_name("password", |view: &mut EditView| view.get_content())
                .unwrap();
            check_credentials(s, &name, &password)
        });

        let login_button = Button::new("login", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            let password = s
                .call_on_name("password", |view: &mut EditView| view.get_content())
                .unwrap();
            check_credentials(s, &name, &password)
        });

        let button_row = LinearLayout::horizontal()
            .child(login_button)
            .child(DummyView.fixed_width(2))
            .child(Button::new("[q]uit", |s| s.quit()));

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

    fn check_credentials(s: &mut Cursive, name: &str, password: &str) {
        s.pop_layer();

        let is_correct = verify(name, password);

        if is_correct {
            select_channel(s, name);
        } else {
            s.add_layer(Dialog::text("Incorrect username or password"));
        }
    }

    fn verify(name: &str, password: &str) -> bool {
        return true;
    }

    fn select_channel(s: &mut Cursive, name: &str) {
        s.pop_layer();

        //let name_copy = name.clone();

        let channel_input = LinearLayout::horizontal()
            .child(TextView::new("Channel name:"))
            .child(EditView::new().with_name("channel_name").fixed_width(24));

        let connect_button = Button::new("Connect", |s| {
            let channel = s
                .call_on_name("channel_name", |view: &mut EditView| view.get_content())
                .unwrap();
            //open_chat(s, &channel, &name)
            open_chat(s, &channel)
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
    fn open_chat(s: &mut Cursive, channel: &str) {
        s.pop_layer();
        //they all reference the same spot on the tui?
        let content = TextContent::new("");
        let message_sender = content.clone();
        let message_receiver = content.clone();
        let (sender, receiver) = mpsc::sync_channel::<String>(0);
        
        
        let _worker = thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(message) => {
                        message_receiver.append('\n'.to_string() + &message.to_string())
                    }
                    Err(mpsc::RecvError) => {
                        return;
                    }
                }
            }
        });
        
        let chat_input = EditView::new().with_name("chat").fixed_width(24);

        let chat_wrapper = LinearLayout::horizontal()
            .child(TextView::new("Chat:"))
            .child(chat_input);

        
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
                sender.send(message.to_string()).unwrap();
            }))
        .child(Button::new("Quit", |s| {
            s.quit();
        }));
        s.add_layer(layout);
    }

    pub fn start_client() {
        let mut siv = cursive::default();

        let connect_button = Button::new("connect", |s| {
            let command = s
                .call_on_name("connect_input", |view: &mut EditView| view.get_content())
                .unwrap();
            connect_to_server(s, &command);
            //return command.to_string();
        });

        let button_row = LinearLayout::horizontal()
            .child(connect_button)
            .child(DummyView.fixed_width(2))
            .child(Button::new("[q]uit", |s| s.quit()));

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

        siv.add_global_callback('q', |s| s.quit());

        siv.run();

    }
}
