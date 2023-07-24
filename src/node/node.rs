use std::{fs};
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::get_temp_folder;

#[derive(Serialize, Deserialize)]
pub struct NodeFile {
    pub name: String,
    pub package_path: PathBuf,
    pub bin: String,
}

pub fn get_nodes() ->Vec<NodeFile> {
    let nodes_file_path = PathBuf::from(get_temp_folder().unwrap()).join("nodes.json");

    if !nodes_file_path.exists() {
        File::create(&nodes_file_path).expect("Cannot create nodes file");
    }

    let nodes_file = fs::read_to_string(nodes_file_path).expect("Could not read nodes file");
    let nodes: Vec<NodeFile> = serde_json::from_str(nodes_file.as_str()).unwrap();

    return nodes;
}