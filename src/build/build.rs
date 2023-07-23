use std::{fs};
use std::fs::{DirEntry, File};
use std::path::PathBuf;
use std::process::{Command};
use cargo_manifest::{Dependency, Manifest};
use crate::package::models::Workspace;
use crate::TEMP_FOLDER;

pub fn build_workspace(workspace: Workspace) {
    let workspace_manifest  = workspace.clone().path.join("Cargo.toml");
    let mut messages_types: Vec<String> = vec![];

    println!("Building workspace...");
    Command::new("cargo").args(["build", "--manifest-path", workspace_manifest.to_str().unwrap()]).output().ok();

    println!("  ◦ Building packages ({})", workspace.packages.len());

    for package_file in workspace.packages {
        let package_path = workspace.path.join(package_file.path.clone());
        let msg_folder_path = workspace.path.join(package_file.path.clone()).join("src").join("msg");

        println!("      ┌—┤ {} ├—┐", package_file.package.name);
        println!("      │");

        if msg_folder_path.exists() && msg_folder_path.is_dir() {
            let message_paths = fs::read_dir(msg_folder_path)
                .unwrap()
                .filter(|entry| !entry.as_ref().unwrap().path().ends_with("mod.rs"))
                .map(|entry| entry.unwrap())
                .collect::<Vec<DirEntry>>();

            println!("      ├— Messages ({})", message_paths.len());

            for path in message_paths {


                println!("      │   - {}", path.file_name().to_str().unwrap());
                println!("      │       - Building", );

                let mut alt_path = path.path();
                alt_path.set_file_name(format!("_{}", path.file_name().to_str().unwrap()));
                fs::copy(path.path(), &alt_path).expect("Could not copy message file");

                append_dependencies_to_message_file(package_path.clone(), alt_path.clone());

                let output = Command::new("cargo").args(["script", alt_path.to_str().unwrap(), TEMP_FOLDER]).output();
                let unwrapped_output = output.unwrap();

                if !unwrapped_output.clone().status.success() {
                    println!("      └———————→ Error");
                    println!();
                    fs::remove_file(alt_path).expect("Could not remove temp message file");
                    panic!("{}", String::from_utf8(unwrapped_output.clone().stderr).unwrap());
                }

                let message_type = String::from_utf8(unwrapped_output.stdout).unwrap();
                messages_types.push(message_type.clone());
                println!("      │       - Done, got \"{}\"", message_type);

                fs::remove_file(alt_path).expect("Could not remove temp message file");
            }
        }

        println!("      │");
        println!("      └———{}—— ·", "—".repeat(package_file.package.name.len()))
    }

    let messages_types_list_file_path = PathBuf::from(TEMP_FOLDER).join("messages_types.json");

    if !messages_types_list_file_path.exists() {
        File::create(&messages_types_list_file_path).expect("Cannot create messages types file");
    }

    let json_messages_types = serde_json::to_string(&messages_types).unwrap();

    fs::write(messages_types_list_file_path, json_messages_types).expect("Could not write in messages types files");

    println!("Workspace built")
}

fn append_dependencies_to_message_file(package_path: PathBuf, message_file: PathBuf) {
    let package_manifest_path = package_path.join("Cargo.toml");
    let package_manifest_content = fs::read_to_string(package_manifest_path).expect("Could not read package Cargo.toml file");
    let package_manifest: Manifest = toml::from_str(package_manifest_content.as_str()).expect("Absent or malformed manifest");

    if package_manifest.dependencies.is_some() {
        let mut dependencies_string =r"//! ```cargo
//! [dependencies]
".to_owned();

        for (name, dependency) in package_manifest.dependencies.unwrap() {
            match dependency {
                Dependency::Simple(version) => {
                    dependencies_string.push_str(format!("//! {} = \"{}\"\n", name.clone(), version.clone()).as_str())
                }
                Dependency::Detailed(detailed) => {
                    if detailed.version.is_some() {
                        dependencies_string.push_str(format!("//! {} = \"{}\"\n", name.clone(), detailed.version.unwrap().clone()).as_str())
                    }

                    else if detailed.path.is_some() {
                        let new_path = detailed.path.unwrap().clone().replace("\\", "\\\\");
                        dependencies_string.push_str(format!("//! {} = {{ path = \"{}\" }}\n", name.clone(), new_path).as_str())
                    }
                }
            }
        }

        dependencies_string.push_str("//! ```\n\n");


        let file_content = fs::read_to_string(&message_file).unwrap();

        fs::write(&message_file, dependencies_string + file_content.as_str()).ok();
    }

}