use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{MAIN_SEPARATOR_STR, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use generic_robot_framework::models::topic::Topic;
use crate::package::PackageType;
use crate::package::workspace::parse_workspace;
use crate::server::message::Message;
use crate::topic::list::handle_message_kind_list;
use crate::topic::tpub::handle_message_kind_pub;
use crate::topic::tsub::handle_message_kind_sub;

pub fn run_server(workspace_path: Option<String>, port: Option<String>) {
    println!("Retrieving workspace information...");

    let path: String;
    if workspace_path.is_some() {
        path = workspace_path.unwrap() + MAIN_SEPARATOR_STR + "Package.toml";
    }
    else {
        path = "Package.toml".to_string();
    }

    let mut workspace = parse_workspace(&path);

    if workspace.package.package_type != PackageType::Workspace {
        panic!("Workspace package type must be 'Workspace' in {}", path)
    }
    else {
        workspace.path = PathBuf::from(path);
    }

    println!(" ===== ");
    /* ===== */
    println!("Starting server...");

    let address = "127.0.0.1:".to_string() + port.unwrap_or("1312".to_string()).as_str();
    let listener = TcpListener::bind(&address).unwrap();

    let topics: Arc<Mutex<Vec<Topic>>> = Arc::new(Mutex::new(vec![
        Topic {
            name: String::from("finish"),
            subscribers: vec![]
        },
        Topic {
            name: String::from("info"),
            subscribers: vec![]
        }
    ]));

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

pub fn handle_connection(mut stream: TcpStream, topics: Arc<Mutex<Vec<Topic>>>) {
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