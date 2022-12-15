use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZfsPartition {
    pool: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Zpool {
    name: Option<String>,
    #[serde(rename="type")]
    type_: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    options: HashMap<String, String>,
    #[serde(default)]
    root_fs_options: HashMap<String, String>,
    mountpoint: Option<String>,
    #[serde(default)]
    mount_options: Vec<String>,
    datasets: HashMap<String, ZfsDataset>
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
    mountpoint: Option<String>,
    #[serde(default)]
    options: HashMap<String, String>,
    #[serde(default)]
    mount_options: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZfsVolume {
    size: String,
    content: HashMap<String, String>,
    #[serde(default)]
    options: HashMap<String, String>,
    #[serde(default)]
    mount_options: Vec<String>,
}
