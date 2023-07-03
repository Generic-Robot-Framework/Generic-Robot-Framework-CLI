use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use generic_robot_framework::models::topic::Topic;
use crate::server::message::Message;
use crate::server::serve::{acknowledgement_http_request, message_to_http_request, OK_HTTP_STATUS, single_request_to_string};

/// Server side sub
pub fn handle_message_kind_sub(mut stream: TcpStream, message: Message, topics: Arc<Mutex<Vec<Topic>>>) {

    let mut topic_exists = false;
    let topic_name = message.topic.as_ref().unwrap().clone();

    for topic in topics.lock().unwrap().iter_mut() {
        if topic.name == topic_name {
            topic_exists = true;

            let new_sub = stream.try_clone().unwrap();

            topic.subscribers.push(new_sub);
        }
    }

    // If topic doesn't exist, create it
    if !topic_exists {
        let new_sub = stream.try_clone().unwrap();

        topics.lock().unwrap().push(Topic {
            name: topic_name,
            subscribers: vec![new_sub],
        })
    }

    let response = acknowledgement_http_request();
    stream.write_all(response.as_bytes()).unwrap();

    println!("Subscribed {} to topic {}", stream.peer_addr().unwrap(), message.topic.unwrap());
}


/// Client side sub
pub fn handle_topic_sub_command(topic_name: &str) {
    println!("Subscribing to topic \"{topic_name}\"");

    let mut stream = TcpStream::connect("127.0.0.1:1312").unwrap();

    let data: Message = Message {
        kind: String::from("sub"),
        topic: Some(String::from(topic_name)),
        message: None,
    };

    let request = message_to_http_request(&data);
    stream.write_all(request.as_bytes()).ok();
    stream.shutdown(Shutdown::Write).ok();

    let response = single_request_to_string(&mut stream);

    if response != OK_HTTP_STATUS {
        panic!("Bad response: {}", response)
    }
    else {
        println!("Subscribed");
    }

    loop {
        let response = single_request_to_string(&mut stream);

        if response.len() > 0 {
            println!("---");
            println!("{response}");
        }
        else {
            break;
        }
    }
}