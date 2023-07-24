use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use crate::message::message::{get_message_type, get_schema, Message, topic_exists};
use crate::server::serve::{acknowledgement_http_request, AtomicTopics, handle_generic_topics, message_to_http_request};

/// Server side topic pub
pub fn handle_message_kind_pub(mut stream: TcpStream, message: Message, topics: AtomicTopics) {
    handle_generic_topics(message.clone().topic.unwrap());

    let mut bytes_to_send = Vec::new();

    if let Some(content) = &message.message.as_ref() {
        bytes_to_send = [bytes_to_send, serde_json::to_vec(content).unwrap()].concat();
    }

    for topic in topics.topics.lock().as_deref_mut().unwrap() {
        if topic.name == message.topic.as_ref().unwrap().clone() {
            topic.write_to_subscribers(&bytes_to_send);
        }
    }

    stream.shutdown(Shutdown::Read).ok();

    let response = acknowledgement_http_request();
    stream.write_all(response.as_bytes()).unwrap();

    println!("Sent message from {} to topic {}", stream.peer_addr().unwrap(), message.topic.unwrap());
}

/// Client side topic pub
pub fn handle_topic_pub_command(topic_name: String, message: Option<String>) {
    if !topic_exists(topic_name.clone()) {
        println!("Topic \"{}\" not found", topic_name);
        exit(1);
    }

    let data: Message;


    let message_type = get_message_type(topic_name.clone()).unwrap();

    if message_type.is_some() {
        if message.is_some() {
            let schema = get_schema(message_type.clone().unwrap());
            let data_to_validate = serde_json::from_str(message.clone().unwrap().as_str()).expect("Could not deserialize message to JSON");
            let result = schema.validate(&data_to_validate);

            if result.is_err() {
                println!("Wrong message format, should be \"{}\" for topic \"{}\"", message_type.unwrap(), topic_name);
                exit(1);
            }

            println!("Sending message \"{}\" to topic \"{}\"", message.clone().unwrap(), topic_name);

            let content = serde_json::from_str(&message.unwrap()).expect("Could not parse message to JSON");

            data = Message {
                kind: String::from("pub"),
                topic: Some(topic_name),
                message_type,
                message: Some(content)
            };
        }
        else {
            println!("Wrong message format, should be \"{}\" for topic \"{}\"", message_type.unwrap(), topic_name);
            exit(1);
        }
    }
    else {
        if message.is_some() {
            println!("Wrong message format, should be None for topic \"{}\"", topic_name);
            exit(1);
        }

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