use clap::{Args, Parser, Subcommand};

mod topic;
mod package;
mod server;

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
    /// Start the topics server
    Serve(Serve),

    /// Topic interaction commands
    #[command(subcommand)]
    Topic(TopicCommands),

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

#[derive(Debug, Subcommand)]
enum TopicCommands {
    /// Topic subscription command
    Sub(SubtCommand),

    /// Topic publication command
    Pub(PubtCommand),

    /// Topic list command
    List(ListCommand),
}

#[derive(Debug, Args)]
struct SubtCommand {
    /// Name of the topic to sub to
    #[arg(value_name = "topic", index = 1)]
    topic: String
}

#[derive(Debug, Args)]
struct PubtCommand {
    /// Name of the topic to pub to
    #[arg(value_name = "topic", index = 1)]
    topic: String,

    /// Message to send
    #[arg(value_name = "message", index = 2)]
    message: Option<String>
}

#[derive(Debug, Args)]
struct ListCommand {

}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Topic( topic_commands) => {
            match topic_commands {
                TopicCommands::Sub(tsub) => {
                    handle_topic_sub_command(tsub.topic.as_str());
                }

                TopicCommands::Pub(mut tput) => {
                    handle_topic_pub_command(tput.topic.as_str(), tput.message.take());
                }

                TopicCommands::List(_) => {
                    handle_topic_list_command();
                }
            }
        }
        Commands::Serve( serve ) => {
            run_server(serve.path, serve.port);
        }
    }
}