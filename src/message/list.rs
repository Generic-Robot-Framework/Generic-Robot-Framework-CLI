use crate::message::message::get_messages_types;

/// Client side list messages types
pub fn handle_message_list_command() {
    let messages_types = get_messages_types();

    println!("Messages:");
    for message_type in messages_types {
        println!("{}", message_type);
    }
}