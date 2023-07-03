use std::fmt::{Debug};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

pub mod workspace;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PackageType {
    Workspace,
    Module,
    Adapter,
    Resource
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageFile {
    pub package: Package,

    #[serde(skip_deserializing)]
    pub path: PathBuf,
}