use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Btrfs {
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub subvolumes: Vec<String>,
}
