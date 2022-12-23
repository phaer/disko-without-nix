use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::disk::Content;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZfsPartition {
    pub pool: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Zpool {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub options: IndexMap<String, String>,
    #[serde(default)]
    pub root_fs_options: IndexMap<String, String>,
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub mount_options: Vec<String>,
    pub datasets: IndexMap<String, ZfsDataset>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "zfs_type")]
#[serde(rename_all = "lowercase")]
pub enum ZfsDataset {
    Filesystem(ZfsFilesystem),
    Volume(ZfsVolume),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZfsFilesystem {
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub options: IndexMap<String, String>,
    #[serde(default)]
    pub mount_options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZfsVolume {
    pub size: String,
    #[serde(default)]
    pub content: Content,
    #[serde(default)]
    pub options: IndexMap<String, String>,
    #[serde(default)]
    pub mount_options: Vec<String>,
}

pub fn make_zfs_options(options: &IndexMap<String, String>, flag: &str) -> String {
    options
        .iter()
        .map(|(n, v)| format!("{} {}={}", flag, n, v))
        .collect::<Vec<String>>()
        .join(" ")
}
