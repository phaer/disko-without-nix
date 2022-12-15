use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub device: String,
    #[serde(rename="type")]
    pub type_: String,
    pub content: DiskContent
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DiskContent {
    Table(Table),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub format: TableFormat,
    pub partitions: Vec<crate::partition::Partition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TableFormat {
    Gpt,
    Msdos
}
