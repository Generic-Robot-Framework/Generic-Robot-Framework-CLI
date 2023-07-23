use clap::{Args, CommandFactory, Parser, Subcommand};
use crate::build::build::build_workspace;
use crate::completions::completions::generate_completions;
use crate::package::workspace::parse_workspace;

mod topic;
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

    Completions
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
    topic: String,

    /// Topic essage type
    #[arg(value_name = "message_type", index = 2)]
    message_type: Option<String>
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

        Commands::Completions => {
            generate_completions(cmd, cmd_name);
        }
    }
}

// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str#:~:text=You%20cannot%20obtain%20%26'static%20str,String%20own%20lifetime%20from%20it.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}