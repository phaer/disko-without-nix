use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    #[serde(rename="type")]
    type_: String,
    #[serde(default)]
    part_type: PartitionType,
    fs_type: Option<FilesystemType>,
    name: Option<String>,
    start: String,
    end: String,
    #[serde(default)]
    flags: Vec<String>,
    #[serde(default)]
    bootable: bool,
    content: PartitionContent
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum PartitionType {
    #[default]
    Primary,
    Logical,
    Extended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FilesystemType {
    Btrfs,
    Ext2,
    Ext3,
    Ext4,
    Fat16,
    Fat32,
    Hfs,
    #[serde(rename="hfs+")]
    HfsPlus,
    #[serde(rename="linux-swap")]
    LinuxSwap,
    Ntfs,
    Reiserfs,
    Udf,
    Xfs
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum PartitionContent {
    Filesystem(FilesystemPartition),
    Zfs(crate::zfs::ZfsPartition),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilesystemPartition {
    format: String,
    mountpoint: String,
    options: Option<Vec<String>>,
    mount_options: Option<Vec<String>>,
    extra_args: Option<String>,
}
