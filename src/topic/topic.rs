use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use jsonschema::JSONSchema;
use crate::TEMP_FOLDER;

pub fn message_type_for_topic(topic_name: String) -> Option<Option<String>> {
    let topics_file_path = PathBuf::from(TEMP_FOLDER).join("topics.json");

    if !topics_file_path.exists() {
        File::create(&topics_file_path).expect("Cannot create topics file");
    }

    let topics_file = fs::read_to_string(topics_file_path).expect("Could not read topics file");
    let topics: HashMap<String, Option<String>> = serde_json::from_str(topics_file.as_str()).unwrap();

    return topics.get(topic_name.as_str()).cloned(); // TODO manage Option<Option<>>
}

pub fn message_type_is_registered(message_type: String) -> bool {
    let messages_types_list_file_path = PathBuf::from(TEMP_FOLDER).join("messages_types.json");

    if !messages_types_list_file_path.exists() {
        File::create(&messages_types_list_file_path).expect("Cannot create messages types file");
    }

    let topics_file = fs::read_to_string(messages_types_list_file_path).expect("Could not read messages types file");
    let topics: Vec<String> = serde_json::from_str(topics_file.as_str()).unwrap();

    for _message_type in topics {
        if _message_type == message_type {
            return true;
        }
    }

    return false;
}

pub fn get_schema(message_type: String) -> JSONSchema {
    let schema_file_path = Path::new(TEMP_FOLDER).join("schemas").join(message_type + ".json");
    let schema_string = fs::read_to_string(schema_file_path).expect("Unable to read message schema file");
    let schema = serde_json::from_str(schema_string.as_str()).unwrap();
    return JSONSchema::compile(&schema).expect("Not a valid schema");
}