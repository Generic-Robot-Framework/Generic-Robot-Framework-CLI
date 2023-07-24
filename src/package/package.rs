use std::fmt::{Debug};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PackageType {
    Module,
    Adapter,
    Resource
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub path: PathBuf,
    pub packages: Vec<PackageFile>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageFile {
    pub package: Package,

    pub path: PathBuf,
}