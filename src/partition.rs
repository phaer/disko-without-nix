use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    #[serde(rename="type")]
    pub type_: String,
    #[serde(default)]
    #[serde(rename="part-type")]
    pub part_type: PartitionType,
    #[serde(rename="fs-type")]
    pub fs_type: Option<FilesystemType>,
    #[serde(default)]
    pub name: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub bootable: bool,
    pub content: crate::disk::Content,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug, Default)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum PartitionType {
    #[default]
    Primary,
    Logical,
    Extended,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
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
#[serde(rename_all = "camelCase")]
pub struct Filesystem {
    pub format: String,
    pub mountpoint: String,
    pub options: Option<Vec<String>>,
    pub mount_options: Option<Vec<String>>,
    pub extra_args: Option<String>,
}

