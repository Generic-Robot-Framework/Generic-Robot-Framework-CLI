use std::io::Write;
use std::net::{Shutdown, TcpStream};
use crate::message::message::{get_message_type, Message};
use crate::server::serve::{AtomicTopics, handle_generic_topics, message_to_http_request};

/// Server side topic pub
pub fn handle_message_kind_pub(stream: TcpStream, message: Message, topics: AtomicTopics) {
    handle_generic_topics(message.clone().topic.unwrap());


    let mut bytes_to_send = Vec::new();

    if let Some(content) = &message.message.as_ref() {
        bytes_to_send = [bytes_to_send, serde_json::to_vec(content).unwrap()].concat();
    }

    for topic in topics.topics.lock().unwrap().iter() {
        if topic.name == message.topic.as_ref().unwrap().clone() {
            topic.write_to_subscribers(&bytes_to_send)
        }
    }

    stream.shutdown(Shutdown::Both).ok();

    println!("Sent message from {} to topic {}", stream.peer_addr().unwrap(), message.topic.unwrap());
}

/// Client side topic pub
pub fn handle_topic_pub_command(topic_name: String, message: Option<String>) {

    let data: Message;

    if message.is_some() {
        println!("Sending message \"{}\" to topic \"{}\"", message.clone().unwrap(), topic_name);

        let message_type = get_message_type(topic_name.clone());

        if message_type.is_none() {
            panic!("Unknown message type");
        }

        let content = serde_json::from_str(&message.unwrap()).expect("Could not parse message to JSON");

        data = Message {
            kind: String::from("pub"),
            topic: Some(topic_name),
            message_type: message_type.unwrap(),
            message: Some(content)
        };
    }
    else {
        println!("Sending empty message to topic \"{}\"", topic_name);

        data = Message {
            kind: String::from("pub"),
            topic: Some(String::from(topic_name)),
            message_type: None,
            message: None
        };
    }

    let mut stream = TcpStream::connect("127.0.0.1:1312").unwrap();

    let request = message_to_http_request(&data);
    stream.write_all(request.as_bytes()).ok();
}