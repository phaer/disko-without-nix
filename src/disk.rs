use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::device::DevicePath;

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum TableFormat {
    Gpt,
    Msdos,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Content {
    #[default]
    None,
    Table(Table),
    Filesystem(crate::partition::Filesystem),
    Zfs(crate::zfs::ZfsPartition),
    Mdraid(crate::mdadm::Mdraid),
    Btrfs(crate::btrfs::Btrfs),
    Swap(crate::swap::Swap),
    Luks(crate::luks::Luks),
    LvmPv(crate::lvm::LvmPv),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub device: DevicePath,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub format: TableFormat,
    pub partitions: Vec<crate::partition::Partition>,
}
