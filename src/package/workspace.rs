use std::fs;
use std::path::{PathBuf};
use cargo_manifest::Manifest;
use crate::package::package::{Package, PackageFile, Workspace};

pub fn parse_workspace(workspace_path: PathBuf) -> Workspace {
    let manifest_path = workspace_path.clone().join("Cargo.toml");

    let manifest_content = fs::read_to_string(&manifest_path).expect("Could not read Cargo.toml file");
    let workspace_manifest: Manifest = toml::from_str(manifest_content.as_str()).expect("Absent or malformed manifest");

    if workspace_manifest.workspace.is_none() {
        panic!("Workspace has no members")
    }

    let toml_workspace = workspace_manifest.workspace.unwrap();

    let mut workspace = Workspace {
        path: PathBuf::from(workspace_path.clone()),
        packages: vec![]
    };

    for package_path in toml_workspace.members {
        let package_manifest_path = workspace_path.clone().join(&package_path).join("Cargo.toml");

        let package_content = fs::read_to_string(package_manifest_path).expect("Could not read Cargo.toml file");
        let package_manifest: Manifest = toml::from_str(package_content.as_str()).expect("Absent or malformed manifest");

        let package = PackageFile {
            package: Package {
                name: package_manifest.package.unwrap().name,
                version: "".to_string()//package_manifest.package.unwrap().version
            },
            path: PathBuf::from(package_path),
        };

        workspace.packages.push(package);
    }

    return workspace;
}