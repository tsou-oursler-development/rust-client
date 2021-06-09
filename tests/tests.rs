use::view::*;

use cursive::views::TextContent;
use std::sync::mpsc;

#[test]
fn test_check_credentials(){
    let mut s = cursive::default();
    let messages = TextContent::new("");
    let (sender, con_rcv) = mpsc::channel();
    match view::check_credentials(&s, &messages, &sender, "server",  "name", "#channel"){
        
    };
}
