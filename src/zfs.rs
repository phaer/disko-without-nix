use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZfsPartition {
    pub pool: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Zpool {
    #[serde(rename="type")]
    pub type_: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
    #[serde(default)]
    pub root_fs_options: HashMap<String, String>,
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub mount_options: Vec<String>,
    pub datasets: HashMap<String, ZfsDataset>
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
    pub options: HashMap<String, String>,
    #[serde(default)]
    pub mount_options: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZfsVolume {
    pub size: String,
    pub content: HashMap<String, String>,
    #[serde(default)]
    pub options: HashMap<String, String>,
    #[serde(default)]
    pub mount_options: Vec<String>,
}
