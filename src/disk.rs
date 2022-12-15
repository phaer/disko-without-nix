use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    device: String,
    #[serde(rename="type")]
     type_: String,
    content: DiskContent
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum DiskContent {
    Table(Table),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    format: TableFormat,
    partitions: Vec<crate::partition::Partition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TableFormat {
    Gpt,
    Msdos
}
