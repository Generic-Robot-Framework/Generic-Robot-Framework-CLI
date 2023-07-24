use crate::node::node::get_nodes;

pub fn list_nodes(with_binary_names: bool, with_package_paths: bool) {
    let nodes = get_nodes();

    let mut separator = "--------------------".to_string();

    print!("{0: <20}", "Node name");

    if with_binary_names {
        print!("{0: <20}", "Binary name");
        separator += "--------------------";
    }

    if with_package_paths {
        print!("{0: <40}", "Package path");
        separator += "----------------------------------------";
    }

    println!();
    println!("{separator}");

    for node in nodes {
        print!("{0: <20}", node.name);

        if with_binary_names {
            print!("{0: <20}", node.bin);
        }

        if with_package_paths {
            print!("{0: <40}", node.package_path.to_str().unwrap());
        }

        println!();
    }
}