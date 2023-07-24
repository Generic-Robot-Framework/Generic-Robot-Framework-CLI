use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use jsonschema::JSONSchema;
use generic_robot_framework::models::topic::Topic;
use crate::message::message::{get_message_type, get_schema, is_message_type_registered, Message, topic_exists};
use crate::server::serve::{acknowledgement_http_request, AtomicTopics, BAD_REQUEST_HTTP_STATUS, message_to_http_request, OK_HTTP_STATUS, single_request_to_string};

/// Server side topic sub
pub fn handle_message_kind_sub(mut stream: TcpStream, message: Message, topics: AtomicTopics) {

    let mut subscribed = false;
    let mut topic_exists = false;
    let topic_name = message.topic.as_ref().unwrap().clone();

    for topic in topics.topics.lock().unwrap().iter_mut() {
        if topic.name == topic_name {
            topic_exists = true;

            if message.message_type == topic.message_type {
                let new_sub = stream.try_clone().unwrap();

                topic.subscribers.push(new_sub);
                subscribed = true;
            }
        }
    }

    // If topic doesn't exist, create it
    if !topic_exists {
        let new_sub = stream.try_clone().unwrap();

        topics.topics.lock().unwrap().push(Topic {
            name: topic_name,
            message_type: message.message_type,
            subscribers: vec![new_sub],
        });

        topics.topics_to_file();

        subscribed = true;
    }

    if  subscribed {
        let response = acknowledgement_http_request();
        stream.write_all(response.as_bytes()).unwrap();
        println!("Subscribed {} to topic {}", stream.peer_addr().unwrap(), message.topic.unwrap());
    }
    else {
        let response = BAD_REQUEST_HTTP_STATUS.to_string();
        stream.write_all(response.as_bytes()).unwrap();
    }
}


/// Client side topic sub
pub fn handle_topic_sub_command(topic_name: String, create_topic_message_type: Option<Option<String>>) {
    println!("Subscribing to topic \"{topic_name}\"");

    let mut stream = TcpStream::connect("127.0.0.1:1312").unwrap();

    let data;
    let validation_schema: Option<JSONSchema>;

    if create_topic_message_type.is_none() {
        if !topic_exists(topic_name.clone()) {
            println!("Topic \"{}\" not found", topic_name);
            exit(1);
        }

        let message_type = get_message_type(topic_name.clone()).unwrap();

        data = Message {
            kind: String::from("sub"),
            topic: Some(topic_name),
            message_type: message_type.clone(),
            message: None,
        };

        if message_type.clone().is_some() {
            validation_schema = Some(get_schema(message_type.unwrap()));
        } else {
            validation_schema = None
        }
    }
    else {
        if create_topic_message_type.clone().unwrap().is_some() {
            if !is_message_type_registered(create_topic_message_type.clone().unwrap().unwrap()) {
                println!("Message type \"{}\" has not been registered", create_topic_message_type.clone().unwrap().unwrap());
                exit(1);
            }

            data = Message {
                kind: String::from("sub"),
                topic: Some(topic_name),
                message_type: create_topic_message_type.clone().unwrap(),
                message: None,
            };

            validation_schema = Some(get_schema(create_topic_message_type.unwrap().unwrap()));
        }
        else {
            data = Message {
                kind: String::from("sub"),
                topic: Some(topic_name),
                message_type: None,
                message: None,
            };

            validation_schema = None;
        }

        println!("Created topic");
    }

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

            dbg!(&response);

            if validation_schema.is_some() {
                let data_to_validate = serde_json::from_str(response.as_str()).unwrap();
                let result = validation_schema.as_ref().unwrap().validate(&data_to_validate);

                if result.is_err() {
                    println!("Got badly formatted message: {}", response)
                }
                else {
                    println!("---");
                    println!("{response}");
                }
            }
            else {
                if response.len() > 1 {
                    println!("Got badly formatted message: {}", response)
                }
                else {
                    println!("---");
                }
            }
        }
        else {
            break;
        }
    }
}