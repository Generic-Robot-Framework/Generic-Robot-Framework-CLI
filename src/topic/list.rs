use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use generic_robot_framework::models::topic::Topic;
use crate::server::message::Message;
use crate::server::serve::{message_to_http_request, single_request_to_string};


/// Server side list
pub fn handle_message_kind_list(mut stream: TcpStream, topics: Arc<Mutex<Vec<Topic>>>) {
    let mut response = String::from("");

    for topic in topics.lock().unwrap().iter() {
        response += &topic.name;
        response.push('\n');
    }

    stream.write_all(response.as_bytes()).ok();
    stream.shutdown(Shutdown::Both).ok();
}

/// Client side list
pub fn handle_topic_list_command() {
    let mut stream = TcpStream::connect("127.0.0.1:1312").unwrap();

    let data = Message {
        kind: String::from("list"),
        topic: None,
        message: None
    };

    let request = message_to_http_request(&data);
    stream.write_all(request.as_bytes()).ok();

    stream.shutdown(Shutdown::Write).ok();

    let response = single_request_to_string(&mut stream);
    print!("Topics:\n{}", response);
}