use std::fs::File;
use std::{fs};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use clap::Command;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use crate::{string_to_static_str, TEMP_FOLDER};

pub fn generate_completions(mut cmd: Command, cmd_name: String) {
    for command in cmd.get_subcommands_mut() {
        if command.get_name() != "topic" {
            continue;
        }

        for topic_command in command.get_subcommands_mut() {
            if topic_command.get_name() != "sub" && topic_command.get_name() != "pub" {
                continue;
            }

            let topics_file_path = PathBuf::from(TEMP_FOLDER).join("topics.json");

            if !topics_file_path.exists() {
                File::create(&topics_file_path).expect("Cannot create topics file");
            }

            let topics_file = fs::read_to_string(topics_file_path).expect("Could not read topics file");
            let topics: HashMap<String, Option<String>> = serde_json::from_str(topics_file.as_str()).unwrap();

            for (topic, message_type) in topics {
                let topic = string_to_static_str(topic);

                if topic.is_empty() {
                    continue;
                }

                let mut new_command = Command::new(topic);

                if message_type.is_some() {
                    let mut message_default = fs::read_to_string(
                        Path::new(TEMP_FOLDER)
                            .join("defaults")
                            .join(message_type.clone().unwrap() + ".json")
                        )
                        .unwrap();

                    #[cfg(windows)]
                    {
                        message_default = message_default.replace("\"", "\\\"");
                    }

                    let message_default = string_to_static_str(format!("\"{}\"", message_default));

                    new_command = new_command.subcommand(Command::new(message_default));
                }

                *topic_command = topic_command.clone().subcommand(new_command);
            }
        }
    }

    let outdir = PathBuf::from(TEMP_FOLDER).join("completions");

    generate_to(Bash, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Elvish, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Fish, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(PowerShell, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Zsh, &mut cmd, &cmd_name, &outdir).ok();
}