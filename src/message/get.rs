use std::process::exit;
use crate::message::message::get_message_type;

/// Client side get message
pub fn handle_get_message_command(topic_name: String) {
    let message_type = get_message_type(topic_name);

    if message_type.is_some() {
        if message_type.clone().unwrap().is_some() {
            println!("{}", message_type.unwrap().unwrap());
        }
        else {
            println!("None");
        }
    }
    else {
        println!("Topic nt found");
        exit(1);
    }
}