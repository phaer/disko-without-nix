use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

mod disk;
mod partition;
mod mdadm;
mod zfs;
mod lvm;
mod nodev;

#[derive(Serialize, Deserialize, Debug)]
struct Devices {
    disk: HashMap<String, disk::Disk>,
    mdadm: Option<HashMap<String, mdadm::Mdadm>>,
    zpool: Option<HashMap<String, zfs::Zpool>>,
    lvm_vg: Option<HashMap<String, lvm::LvmVg>>,
    nodev: Option<HashMap<String, nodev::Nodev>>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Please pass a json file as the first argument.")
    };
    let config_file = std::fs::read_to_string(filename)?;
    let config: Devices = serde_json::from_str(&config_file)?;

    println!("{:#?}", config);

    Ok(())
}
