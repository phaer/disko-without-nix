use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub device: String,
    #[serde(rename="type")]
    pub type_: String,
    pub content: Content
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Content {
    Table(Table),
    Filesystem(crate::partition::Filesystem),
    Zfs(crate::zfs::ZfsPartition),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub format: TableFormat,
    pub partitions: Vec<crate::partition::Partition>
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum TableFormat {
    Gpt,
    Msdos
}
