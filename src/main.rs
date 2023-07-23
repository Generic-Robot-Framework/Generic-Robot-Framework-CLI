use clap::{Args, CommandFactory, Parser, Subcommand};
use crate::build::build::build_workspace;
use crate::completions::completions::generate_completions;
use crate::message::get::handle_get_message_command;
use crate::message::list::handle_message_list_command;
use crate::message::show::handle_show_message_command;
use crate::package::workspace::parse_workspace;

mod topic;
mod message;
mod package;
mod server;
mod build;
mod completions;

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

    /// Topic message type
    #[arg(value_name = "message_type", index = 2)]
    message_type: Option<String>
}

#[derive(Debug, Args)]
struct PubTopicCommand {
    /// Name of the topic to pub to
    #[arg(value_name = "topic", index = 1)]
    topic: String,

    /// Message to send
    #[arg(value_name = "message", index = 2)]
    message: Option<String>
}

#[derive(Debug, Args)]
struct ListTopicCommand {

}

#[derive(Debug, Subcommand)]
enum MsgCommands {
    /// Get message type for the given topic
    Get(GetMsgCommand),

    /// Show default data for the given message type
    Show(ShowMsgCommand),

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
struct ListMsgCommand {

}

const TEMP_FOLDER: &str = "C:\\Users\\Julien\\Documents\\Recherche\\Generic_Robot_Framework\\temp\\";

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();
    let cmd_name = "grf".to_string();
    cmd.set_bin_name(&cmd_name);

    match cli.command {
        Commands::Topic(topic_commands) => {
            match topic_commands {
                TopicCommands::Sub(tsub) => {
                    handle_topic_sub_command(tsub.topic, tsub.message_type);
                }

                TopicCommands::Pub(mut tpub) => {
                    handle_topic_pub_command(tpub.topic, tpub.message.take());
                }

                TopicCommands::List(_) => {
                    handle_topic_list_command();
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
                MsgCommands::List(_list) => {
                    handle_message_list_command()
                }
            }
        }

        Commands::Build(build) => {
            let path;

            if build.path.is_some() {
                path = build.path.unwrap();
            }
            else {
                path = std::env::current_dir().unwrap().display().to_string()
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