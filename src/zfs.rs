use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZfsPartition {
    pub pool: String,
}

impl ZfsPartition {
    pub fn create(&self, device: &str) -> Vec<String> {
        vec![
            format!(
                "ZFSDEVICES_{}=\"${{ZFSDEVICES_{}:-}}{} \"",
                &self.pool,
                &self.pool,
                device
            )
        ]
    }
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

impl Zpool {
    pub fn create(&self, zpool_name: &str) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        commands.push(format!(
            "zpool create {} {} {} {} ${{ZFSDEVICES_{}}}",
            zpool_name,
            self.mode,
            make_zfs_options(&self.options, "-o"),
            make_zfs_options(&self.root_fs_options, "-O"),
            zpool_name,
        ));

        for (dataset_name, dataset_config) in &self.datasets {
            commands.append(&mut match dataset_config {
                ZfsDataset::Filesystem(filesystem) => filesystem.create(zpool_name, dataset_name),
                ZfsDataset::Volume(volume) => volume.create(zpool_name, dataset_name)
            })
        }
        commands
    }
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

impl ZfsFilesystem {
    pub fn create(&self, zpool_name: &str, dataset_name: &str) -> Vec<String> {
        vec![format!(
            "zfs create {}/{} {}",
            zpool_name,
            dataset_name,
            make_zfs_options(&self.options, "-o")
        )]
    }
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

impl ZfsVolume {
    pub fn create(&self, zpool_name: &str, dataset_name: &str) -> Vec<String> {
        vec![
            format!(
                "zfs create {}/{} {} -V {}",
                zpool_name,
                dataset_name,
                make_zfs_options(&self.options, "-o"),
                self.size
            ),
            String::from("udevadm trigger --subsystem-match=block; udevadm settle")
            // TODO create volume contents
        ]
    }
}

pub fn make_zfs_options(options: &HashMap<String, String>, flag: &str) -> String {
    options.iter().map(|(n, v)| format!("{} {}={}", flag, n, v)).collect::<Vec<String>>().join(" ")
}

