use std::io::{BufRead, BufReader};
use std::process::{Command, exit, Stdio};
use crate::node::node::get_nodes;


pub fn run_node(node_name: String) {
    let nodes = get_nodes();

    for node in nodes {
        if node.name == node_name {
            let mut cmd = Command::new("cargo")
                .args([
                    "run",
                    "--manifest-path", node.package_path.clone().join("Cargo.toml").to_str().unwrap(),
                    "--bin", &node.bin,
                ])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines {
                    println!("{}", line.unwrap());
                }
            }

            cmd.wait().unwrap();
            exit(0);
        }
    }

    panic!("Node {} not found in registered nodes", node_name);
}
