use std::io::Write;
use std::net::{Shutdown, TcpStream};
use crate::message::message::{get_topics};
use crate::server::serve::{AtomicTopics};


/// Server side topic list
pub fn handle_message_kind_list(mut stream: TcpStream, topics: AtomicTopics) {
    let mut response = String::from("");

    for topic in topics.topics.lock().unwrap().iter() {
        response += "- ";
        response += &topic.name;
        response.push('\n');
    }

    stream.write_all(response.as_bytes()).ok();
    stream.shutdown(Shutdown::Both).ok();
}

/// Client side topic list
pub fn handle_topic_list_command(with_message_types: bool) {
    let topics = get_topics();

    let mut separator = "--------------------".to_string();

    print!("{0: <20}", "Topic name");

    if with_message_types {
        print!("{0: <20}", "Message type");
        separator += "--------------------";
    }

    println!();
    println!("{separator}");

    for (topic, message_type) in topics {
        print!("{0: <20}", topic);

        if with_message_types {
            if message_type.is_some() {
                print!("{0: <20}", message_type.unwrap());
            }
            else {
                print!("{0: <20}", "None")
            }
        }

        println!();
    }
}