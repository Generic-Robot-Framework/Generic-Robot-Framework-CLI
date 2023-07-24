use std::{fs};
use std::fs::{DirEntry, File};
use std::path::PathBuf;
use std::process::{Command, exit};
use crate::get_temp_folder;
use crate::package::package::Workspace;
use crate::node::node::NodeFile;

pub fn build_workspace(workspace: Workspace) {
    let workspace_manifest  = workspace.clone().path.join("Cargo.toml");
    let mut messages_types: Vec<String> = vec![];
    let mut nodes: Vec<NodeFile> = vec![];

    println!("Building workspace...");
    Command::new("cargo").args(["build", "--manifest-path", workspace_manifest.to_str().unwrap()]).output().ok();

    println!("  ◦ Building packages ({})", workspace.packages.len());

    for package_file in workspace.packages {
        let package_path = workspace.path.join(package_file.path.clone());
        let msg_folder_path = workspace.path.join(package_file.path.clone()).join("src").join("msg");
        let bin_folder_path = workspace.path.join(package_file.path.clone()).join("src").join("bin");

        println!("      ┌—┤ {} ├—┐", package_file.package.name);

        build_messages(msg_folder_path, bin_folder_path.clone(), package_path.clone(), &mut messages_types);
        build_binaries(bin_folder_path, package_path.clone(), &mut nodes);

        println!("      │");
        println!("      └———{}—— ·", "—".repeat(package_file.package.name.len()))
    }

    write_messages_types_to_file(messages_types);
    write_nodes_to_file(nodes);

    println!("Workspace built")
}

fn build_messages(msg_folder_path: PathBuf, bin_folder_path: PathBuf, package_path: PathBuf, messages_types: &mut Vec<String>) {
    if msg_folder_path.exists() && msg_folder_path.is_dir() {
        let message_paths = fs::read_dir(msg_folder_path)
            .unwrap()
            .filter(|entry| !entry.as_ref().unwrap().path().ends_with("mod.rs"))
            .map(|entry| entry.unwrap())
            .collect::<Vec<DirEntry>>();

        println!("      │");
        println!("      ├— Messages ({})", message_paths.len());

        for path in message_paths {
            println!("      │   - {}", path.file_name().to_str().unwrap());
            println!("      │       - Building", );

            let alt_path = bin_folder_path.clone().join(format!("_{}", path.file_name().to_str().unwrap()));
            fs::copy(path.path(), &alt_path).expect("Could not copy message file");

            let bin_name = alt_path.file_stem().unwrap().to_str().unwrap();

            let output = Command::new("cargo")
                .args([
                    "run",
                    "--manifest-path", package_path.clone().join("Cargo.toml").to_str().unwrap(),
                    "--bin", bin_name.clone(),
                    get_temp_folder().unwrap().as_str()
                ]).output();
            let unwrapped_output = output.unwrap();

            if !unwrapped_output.clone().status.success() {
                print_error("Compilation failed".to_string());
                fs::remove_file(alt_path).expect("Could not remove temp message file");
                println!("{}", String::from_utf8(unwrapped_output.clone().stderr).unwrap());
                exit(1);
            }

            let message_type = String::from_utf8(unwrapped_output.stdout).unwrap();

            if messages_types.iter().find(|&node| node == &message_type).is_some() {
                print_error(format!("A message with name \"{}\" already exists", message_type));
                exit(1);
            }

            messages_types.push(message_type.clone());
            println!("      │       - Done, got message \"{}\"", message_type);

            fs::remove_file(alt_path).expect("Could not remove temp message file");
        }
    }
}

fn build_binaries(bin_folder_path: PathBuf, package_path: PathBuf, nodes: &mut Vec<NodeFile>) {

    if bin_folder_path.exists() && bin_folder_path.is_dir() {
        let bin_paths = fs::read_dir(bin_folder_path)
            .unwrap()
            .filter(|entry| !entry.as_ref().unwrap().path().ends_with("mod.rs"))
            .map(|entry| entry.unwrap())
            .collect::<Vec<DirEntry>>();

        println!("      │");
        println!("      ├— Binaries ({})", bin_paths.len());

        for path in bin_paths {


            println!("      │   - {}", path.file_name().to_str().unwrap());
            println!("      │       - Building", );

            let path = path.path();
            let bin_name = path.file_stem().unwrap().to_str().unwrap();

            let output = Command::new("cargo")
                .args([
                    "run",
                    "--manifest-path", package_path.clone().join("Cargo.toml").to_str().unwrap(),
                    "--bin", bin_name.clone(),
                    "build"
                ]).output();
            let unwrapped_output = output.unwrap();

            if !unwrapped_output.clone().status.success() {
                print_error("Compilation failed".to_string());
                println!("{}", String::from_utf8(unwrapped_output.clone().stderr).unwrap());
                exit(1);
            }

            let node_name = String::from_utf8(unwrapped_output.stdout).unwrap();

            if nodes.iter().find(|&node| node.name == node_name).is_some() {
                print_error(format!("A node with name \"{}\" already exists", node_name));
                exit(1);
            }

            nodes.push(NodeFile {
                name: node_name.clone(),
                package_path: package_path.clone(),
                bin: bin_name.to_string(),
            });
            println!("      │       - Done, got node \"{}\"", node_name);
        }
    }
}

fn write_messages_types_to_file(messages_types: Vec<String>) {
    let messages_types_list_file_path = PathBuf::from(get_temp_folder().unwrap()).join("messages_types.json");

    if !messages_types_list_file_path.exists() {
        File::create(&messages_types_list_file_path).expect("Cannot create messages types file");
    }

    let json_messages_types = serde_json::to_string(&messages_types).unwrap();

    fs::write(messages_types_list_file_path, json_messages_types).expect("Could not write in messages types file");
}

fn write_nodes_to_file(nodes: Vec<NodeFile>) {
    let nodes_list_file_path = PathBuf::from(get_temp_folder().unwrap()).join("nodes.json");

    if !nodes_list_file_path.exists() {
        File::create(&nodes_list_file_path).expect("Cannot create node file");
    }

    let json_nodes = serde_json::to_string(&nodes).unwrap();

    fs::write(nodes_list_file_path, json_nodes).expect("Could not write in nodes file");
}

fn print_error(error: String) {
    println!("      │       - Error");
    println!("      │");
    println!("      └———————→ {error}");
    println!();
}