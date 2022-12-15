use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    #[serde(rename="type")]
    pub type_: String,
    #[serde(default)]
    pub part_type: PartitionType,
    pub fs_type: Option<FilesystemType>,
    pub name: Option<String>,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub bootable: bool,
    pub content: PartitionContent
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
    pub format: String,
    pub mountpoint: String,
    pub options: Option<Vec<String>>,
    pub mount_options: Option<Vec<String>>,
    pub extra_args: Option<String>,
}
