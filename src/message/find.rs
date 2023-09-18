use crate::message::message::get_topics;

/// Client side find messages type usage
pub fn handle_message_find_command(message_type: String) {
    let topics = get_topics();

    for topic in topics {
        if topic.1.is_some() && topic.1.unwrap() == message_type {
            println!("{}", topic.0)
        }
    }
}