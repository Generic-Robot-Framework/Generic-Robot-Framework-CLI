use std::{fs};
use std::io::{stdin};
use std::path::PathBuf;
use std::process::exit;
use clap::{Args, CommandFactory, Parser, Subcommand};
use directories::BaseDirs;

#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(windows)]
use winreg::RegKey;

use crate::build::build::build_workspace;
use crate::completions::completions::generate_completions;
use crate::message::find::handle_message_find_command;
use crate::message::get::handle_get_message_command;
use crate::message::list::handle_message_list_command;
use crate::message::show::handle_show_message_command;
use crate::package::workspace::parse_workspace;
use crate::node::list::list_nodes;
use crate::node::run::run_node;

mod topic;
mod message;
mod package;
mod server;
mod build;
mod completions;
mod node;

use crate::server::serve::run_server;
use crate::topic::list::{handle_topic_list_command};
use crate::topic::tpub::{handle_topic_pub_command};
use crate::topic::tsub::{handle_topic_sub_command};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum  Commands {
    /// Build the workspace
    Build(Build),

    /// Start the topics server
    Serve(Serve),

    /// Node interaction commands
    #[command(subcommand)]
    Node(NodeCommands),

    /// Topic interaction commands
    #[command(subcommand)]
    Topic(TopicCommands),

    /// Messages interaction commands
    #[command(subcommand)]
    Msg(MsgCommands),

    /// Creates the completion files to source in order to use topics and default messages
    Completions(Completions)
}

#[derive(Debug, Args)]
struct Build {
    /// Optional, serve a workspace from outside
    #[arg(long)]
    path: Option<String>
}

#[derive(Debug, Args)]
struct Serve {
    /// Optional, serve with a specific port
    #[arg(short, long)]
    port: Option<String>,

    /// Optional, serve a workspace from outside
    #[arg(long)]
    path: Option<String>
}

#[derive(Debug, Args)]
struct Completions {
    /// Avoid sourcing the file after it's generated
    #[arg(short, long)]
    no_sourcing: bool,
}

#[derive(Debug, Subcommand)]
enum  NodeCommands {
    /// Run the given registered node
    Run(RunNodeCommand),

    /// List the registered nodes
    List(ListNodeCommand)
}

#[derive(Debug, Args)]
struct RunNodeCommand {
    /// Name of the node to run
    #[arg(value_name = "node_name", index = 1)]
    node_name: String,
}

#[derive(Debug, Args)]
struct ListNodeCommand {
    /// Also print binary names
    #[arg(short, long)]
    bin_name: bool,

    /// Also print package path
    #[arg(short, long)]
    package_path: bool,
}

#[derive(Debug, Subcommand)]
enum TopicCommands {
    /// Topic subscription command
    Sub(SubTopicCommand),

    /// Topic publication command
    Pub(PubTopicCommand),

    /// Topic list command
    List(ListTopicCommand),
}

#[derive(Debug, Args)]
struct SubTopicCommand {
    /// Name of the topic to sub to
    #[arg(value_name = "topic", index = 1)]
    topic: String,

    /// Create a topic with given message type, None if no message type was provided
    #[arg(short, long, value_name = "message_type", default_missing_value = None, required = false)]
    create_topic: Option<Option<String>>,
}

#[derive(Debug, Args)]
struct PubTopicCommand {
    /// Name of the topic to pub to
    #[arg(value_name = "topic", index = 1)]
    topic: String,

    /// Message to send
    #[arg(value_name = "message", index = 2)]
    message: Option<String>,
}

#[derive(Debug, Args)]
struct ListTopicCommand {
    /// Also prints messages types
    #[arg(short, long)]
    message_types: bool,
}

#[derive(Debug, Subcommand)]
enum MsgCommands {
    /// Get message type for the given topic
    Get(GetMsgCommand),

    /// Show default data for the given message type
    Show(ShowMsgCommand),

    /// Find the topics that use the given message type
    Find(FindMsgCommand),

    /// List registered messages
    List(ListMsgCommand)
}

#[derive(Debug, Args)]
struct GetMsgCommand {
    /// Name of the topic to retrieve message type
    #[arg(value_name = "topic", index = 1)]
    topic: String,
}

#[derive(Debug, Args)]
struct ShowMsgCommand {
    /// Name of the message type to show default data
    #[arg(value_name = "message_type", index = 1)]
    message_type: String,

    /// Pretty print
    #[arg(short, long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct FindMsgCommand {
    /// Name of the message type to find usage of
    #[arg(value_name = "message_type", index = 1)]
    message_type: String,
}

#[derive(Debug, Args)]
struct ListMsgCommand {

}

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();
    let cmd_name = "grf".to_string();
    cmd.set_bin_name(&cmd_name);

    verify_env_variable();

    match cli.command {
        Commands::Topic(topic_commands) => {
            match topic_commands {
                TopicCommands::Sub(tsub) => {
                    handle_topic_sub_command(tsub.topic, tsub.create_topic);
                }

                TopicCommands::Pub(mut tpub) => {
                    handle_topic_pub_command(tpub.topic, tpub.message.take());
                }

                TopicCommands::List(list) => {
                    handle_topic_list_command(list.message_types);
                }
            }
        }

        Commands::Msg(message_commands) => {
            match message_commands {
                MsgCommands::Get(get) => {
                    handle_get_message_command(get.topic);
                }
                MsgCommands::Show(show) => {
                    handle_show_message_command(show.message_type, show.pretty)
                }
                MsgCommands::Find(find) => {
                    handle_message_find_command(find.message_type)
                }
                MsgCommands::List(_list) => {
                    handle_message_list_command()
                }
            }
        }

        Commands::Node(node) => {
            match node {
                NodeCommands::Run(run) => {
                    run_node(run.node_name)
                }

                NodeCommands::List(list) => {
                    list_nodes(list.bin_name, list.package_path)
                }
            }
        }

        Commands::Build(build) => {
            let path;

            if build.path.is_some() {
                path = PathBuf::from(build.path.unwrap());
            }
            else {
                path = std::env::current_dir().unwrap();
            }

            let workspace = parse_workspace(path);
            build_workspace(workspace);
        }

        Commands::Serve(serve) => {
            run_server(serve.port);
        }

        Commands::Completions(completions) => {
            generate_completions(cmd, cmd_name, completions.no_sourcing);
        }
    }
}

// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str#:~:text=You%20cannot%20obtain%20%26'static%20str,String%20own%20lifetime%20from%20it.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn verify_env_variable() {
    let temp_folder = get_temp_folder();

    if temp_folder.is_err() {
        println!("Environment variable \"GRF_TEMP_FOLDER\" is not set");
        println!("Would you like to set it automatically? [yes/no]");

        let mut response = String::new();
        stdin().read_line(&mut response).unwrap();
        response = response.replace("\n", "").replace("\r", "");

        if response == "yes" || response == "y" {

            if let Some(base_dirs) = BaseDirs::new() {
                let temp_folder = base_dirs.config_local_dir().join("grf");

                if !temp_folder.exists() {
                    fs::create_dir(temp_folder.clone()).expect("Could not create GRF config folder");
                }

                let completions_folder = temp_folder.clone().join("completions");
                if !completions_folder.exists() {
                    fs::create_dir(completions_folder).expect("Could not create GRF completions folder");
                }

                let defaults_folder = temp_folder.clone().join("defaults");
                if !defaults_folder.exists() {
                    fs::create_dir(defaults_folder).expect("Could not create GRF defaults folder");
                }

                let schema_folder = temp_folder.clone().join("schemas");
                if !schema_folder.exists() {
                    fs::create_dir(schema_folder).expect("Could not create GRF schemas folder");
                }

                #[cfg(windows)]
                {
                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    let (env, _) = hkcu.create_subkey("Environment").unwrap();
                    env.set_value("GRF_TEMP_FOLDER", &temp_folder.to_str().unwrap()).expect("Cannot set GRF_TEMP_FOLDER environment variable");
                }
                #[cfg(not(windows))] {
                    std::env::set_var("GRF_TEMP_FOLDER", temp_folder.to_str().unwrap());
                }

                println!("Successfully created temps dir at location:");
                println!("{}", temp_folder.to_str().unwrap());
                println!();
                println!("You can now use the CLI");
                exit(0);
            }
        }

        println!("You need to set the variable before using the CLI");
        exit(1);
    }
}

#[cfg(windows)]
fn get_temp_folder() -> std::io::Result<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    env.get_value("GRF_TEMP_FOLDER")
}

#[cfg(not(windows))]
fn get_temp_folder() -> std::io::Result<String> {
    let result = std::env::var("GRF_TEMP_FOLDER");

    return if let Some(error) = result.clone().err() {
        Err(std::io::Error::new(ErrorKind::Other, error))
    } else {
        Ok(result.unwrap())
    }
}