use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use generic_robot_framework::models::topic::Topic;
use crate::server::message::Message;
use crate::TEMP_FOLDER;
use crate::topic::list::handle_message_kind_list;
use crate::topic::tpub::handle_message_kind_pub;
use crate::topic::tsub::handle_message_kind_sub;

pub fn run_server(port: Option<String>) {

    /* ===== */
    println!("Starting server...");

    let address = "127.0.0.1:".to_string() + port.unwrap_or("1312".to_string()).as_str();
    let listener = TcpListener::bind(&address).unwrap();

    let topics = AtomicTopics::new(Arc::new(Mutex::new(vec![
        Topic {
            name: String::from("finish"),
            message_type: None,
            subscribers: vec![]
        },
        Topic {
            name: String::from("info"),
            message_type: None,
            subscribers: vec![]
        }
    ])));

    topics.topics_to_file();


    println!("Server started on: {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let topics_local_state = topics.clone();
                thread::spawn(move || handle_connection(stream, topics_local_state))
                    .join()
                    .ok();
            }
            Err(_) => {
                println!("Error");
            }
        }

    }
}

pub fn handle_connection(mut stream: TcpStream, topics: AtomicTopics) {
    let http_request = single_request_to_string_vec(&mut stream);

    println!("Received: {:?}", http_request.to_vec());

    if http_request.len() != 3 {
        panic!("Malformed request")
    }

    if http_request[0] != OK_HTTP_STATUS {
        panic!("Malformed status line")
    }

    if !http_request[2].starts_with("Content: ") {
        panic!("Malformed content")
    }

    let content = &http_request[2]["Content: ".len()..];
    let message: Message = serde_json::from_str(content).expect("Malformed message");

    match message.kind.as_str() {
        "sub" => {
            handle_message_kind_sub(stream, message, topics)
        }
        "pub" => {
            handle_message_kind_pub(stream, message, topics)
        }
        "list" => {
            handle_message_kind_list(stream, topics)
        }
        _ => {
            panic!("Unknown message kind")
        }
    }
}

/// Handle topics that are generic
pub fn handle_generic_topics(topic_name: String) {
    if topic_name.as_str() == "finish" {
        println!("Closing");
        exit(0)
    }
}

pub const OK_HTTP_STATUS: &str = "HTTP/1.1 200 OK";

/// Returns a single request and returns it a Vector of Strings
pub fn single_request_to_string_vec(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

/// Returns a single request and returns it a String
pub fn single_request_to_string(stream: &mut TcpStream) -> String {
    let mut buf = vec![0u8; 1024];
    let mut handle = stream.try_clone().unwrap().take(1024);
    handle.read(&mut buf).unwrap();

    String::from_utf8_lossy(&buf)
        .chars()
        .take_while(|&ch| ch != '\0')
        .collect::<String>()
}

/// Acknowledgement HTTP request
pub fn acknowledgement_http_request() -> String {
    OK_HTTP_STATUS.to_string()
}

/// Formatting a String to a HTTP request
#[allow(dead_code)]
pub fn string_to_http_request(data: String) -> String {
    let length = data.len();

    format!("{OK_HTTP_STATUS}\r\nContent-Length: {length}\r\nContent: {data}")
}

/// Formatting a &Message to a HTTP request
pub fn message_to_http_request(data: &Message) -> String {
    let contents = serde_json::to_string(data).unwrap();
    let length = contents.len();

    format!("{OK_HTTP_STATUS}\r\nContent-Length: {length}\r\nContent: {contents}")
}


#[derive(Clone)]
pub struct AtomicTopics {
    pub(crate) topics: Arc<Mutex<Vec<Topic>>>
}

impl AtomicTopics {
    const fn new(topics: Arc<Mutex<Vec<Topic>>>) -> AtomicTopics {
        AtomicTopics {
            topics
        }
    }

    /// Writes the name of the available topics to the topics file
    pub fn topics_to_file(&self) {
        let topics_file_path = PathBuf::from(TEMP_FOLDER).join("topics.json");

        let mut topics_file = File::create(topics_file_path).expect("Cannot create topics file");

        let topics: HashMap<String, Option<String>> = self.topics
            .lock()
            .unwrap()
            .iter()
            .map(|topic| (topic.name.clone(), topic.message_type.clone()))
            .collect();

        let json_topics = serde_json::to_string(&topics).unwrap();

        topics_file.write_all(json_topics.as_bytes()).ok();
    }
}

