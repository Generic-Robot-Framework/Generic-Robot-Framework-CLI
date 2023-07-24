use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use jsonschema::JSONSchema;
use crate::get_temp_folder;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub kind: String,
    pub topic: Option<String>,
    pub message_type: Option<String>,
    pub message: Option<Value>,
}


pub fn get_messages_types() -> Vec<String> {
    let messages_types_list_file_path = PathBuf::from(env::var("GRF_TEMP_FOLDER").unwrap()).join("messages_types.json");

    if !messages_types_list_file_path.exists() {
        File::create(&messages_types_list_file_path).expect("Cannot create messages types file");
    }

    let messages_types_files = fs::read_to_string(messages_types_list_file_path).expect("Could not read messages types file");
    let messages_types: Vec<String> = serde_json::from_str(messages_types_files.as_str()).unwrap();

    return messages_types;
}

pub fn get_topics() -> HashMap<String, Option<String>> {
    let topics_file_path = PathBuf::from(get_temp_folder().unwrap()).join("topics.json");

    if !topics_file_path.exists() {
        File::create(&topics_file_path).expect("Cannot create topics file");
    }

    let topics_file = fs::read_to_string(topics_file_path).expect("Could not read topics file");
    let topics: HashMap<String, Option<String>> = serde_json::from_str(topics_file.as_str()).unwrap();

    return topics;
}

pub fn get_schema(message_type: String) -> JSONSchema {
    let schema_file_path = Path::new(get_temp_folder().unwrap().as_str()).join("schemas").join(message_type + ".json");
    let schema_string = fs::read_to_string(schema_file_path).expect("Unable to read message schema file");
    let schema = serde_json::from_str(schema_string.as_str()).unwrap();
    return JSONSchema::compile(&schema).expect("Not a valid schema");
}

pub fn get_default(message_type: String) -> Option<String> {
    let message_default_result = fs::read_to_string(
        Path::new(get_temp_folder().unwrap().as_str())
            .join("defaults")
            .join(message_type + ".json")
    );

    if message_default_result.is_err() {
        return None;
    }

    let mut message_default = message_default_result.unwrap();

    #[cfg(windows)]
    {
        message_default = message_default.replace("\"", "\\\"");
    }

    return Some(message_default);
}

pub fn get_message_type(topic_name: String) -> Option<Option<String>> {
    let topics = get_topics();

    return topics.get(topic_name.as_str()).cloned();
}


pub fn topic_exists(topic_name: String) -> bool {
    let topics = get_topics();

    for (topic, _) in topics {
        if topic == topic_name {
            return true;
        }
    }

    return false;
}

pub fn is_message_type_registered(message_type: String) -> bool {

    let registered_messages_types = get_messages_types();

    for registered_message_type in registered_messages_types {
        if registered_message_type == message_type {
            return true;
        }
    }

    return false;
}