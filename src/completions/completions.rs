use std::path::{PathBuf};
use clap::Command;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use crate::{get_temp_folder, string_to_static_str};
use crate::message::message::{get_default, get_topics};
use crate::node::node::get_nodes;

pub fn generate_completions(mut cmd: Command, cmd_name: String, no_sourcing: bool) {
    for command in cmd.get_subcommands_mut() {

        match command.get_name() {
            "topic" => {
                for topic_command in command.get_subcommands_mut() {
                    match topic_command.get_name() {
                        "sub" | "pub" => {
                            let topics = get_topics();

                            for (topic, message_type) in topics {
                                let topic = string_to_static_str(topic);

                                if topic.is_empty() {
                                    continue;
                                }

                                let mut new_command = Command::new(topic);

                                if message_type.is_some() {
                                    let message_default = get_default(message_type.unwrap());

                                    if message_default.is_some() {
                                        let message_default = string_to_static_str(format!("\"{}\"", message_default.unwrap()));

                                        new_command = new_command.subcommand(Command::new(message_default));
                                    }
                                }

                                *topic_command = topic_command.clone().subcommand(new_command);
                            }
                        }
                        _ => {}
                    }
                }
            }
            "node" => {
                for node_command in command.get_subcommands_mut() {
                    match node_command.get_name() {
                        "run" => {
                            let nodes = get_nodes();

                            for node in nodes {
                                let node_name = string_to_static_str(node.name);

                                if node_name.is_empty() {
                                    continue;
                                }

                                let new_command = Command::new(node_name);

                                *node_command = node_command.clone().subcommand(new_command);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let outdir = PathBuf::from(get_temp_folder().unwrap()).join("completions");

    let _powershell = generate_to(PowerShell, &mut cmd, &cmd_name, &outdir).ok();
    let _bash = generate_to(Bash, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Elvish, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Fish, &mut cmd, &cmd_name, &outdir).ok();
    generate_to(Zsh, &mut cmd, &cmd_name, &outdir).ok();

    println!("Output folder:");
    println!("{}", outdir.to_str().unwrap());

    if !no_sourcing {

        todo!();
        /*
        #[cfg(windows)]
        {
            let mut ps = std::process::Command::new("powershell.exe")
                .arg("-c")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Could not source completion file");

            let stdin = ps.stdin.as_mut().unwrap();
            stdin.write_all(powershell.unwrap().to_str().unwrap().as_bytes());
        }
        #[cfg(linux)]
        {
            let shell = env!("$SHELL");
            std::process::Command::new("source")
                .arg(bash.unwrap().to_str().unwrap())
                .output();
        }*/
    }
}