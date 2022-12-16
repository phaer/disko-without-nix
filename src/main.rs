use std::collections::HashMap;
use std::convert::AsRef;
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

    //println!("{:#?}", config);

    let mut create_cmds: Vec<String>  = Vec::new();

    for (_disk_name, disk_config) in &config.disk {
        match &disk_config.content {
            disk::Content::Table(table) => {
                create_cmds.push(format!(
                    "parted -s {} -- mklabel {}",
                    disk_config.device,
                    table.format.as_ref(),
                ));

                let mut index = 0;
                for partition in &table.partitions {
                    index += 1;
                    let fs_type = partition.fs_type.as_ref().map_or_else(|| "", |v| v.as_ref());
                    let mut args = Vec::new();
                    match table.format {
                        disk::TableFormat::Gpt => args.push(partition.name.as_str()),
                        disk::TableFormat::Msdos => {
                            args.push(partition.part_type.as_ref());
                            args.push(fs_type);
                        }
                    }
                    args.append(&mut vec![
                        fs_type, &partition.start, &partition.end
                    ]);

                    create_cmds.push(format!(
                        "parted -s {} -- mkpart {} ",
                        disk_config.device,
                        args.join(" ")
                    ));

                    create_cmds.push("# ensure /dev/disk/by-path/..-partN exists before continuing".to_string());
                    create_cmds.push("udevadm trigger --subsystem-match=block; udevadm settle".to_string());

                    if partition.bootable {
                        create_cmds.push(format!(
                            "parted -s {} -- set {} boot on",
                            disk_config.device,
                            index
                        ));
                    }

                    for flag in &partition.flags {
                        create_cmds.push(format!(
                            "parted -s {} -- set {} {} on",
                            disk_config.device,
                            index,
                            flag
                        ));
                    }

                    create_cmds.push("# ensure further operations can detect new partitions".to_string());
                    create_cmds.push("udevadm trigger --subsystem-match=block; udevadm settle".to_string());

                    let device = format!("{}{}", disk_config.device, index);  // TODO port deviceNumbering
                    let create_content_cmds = match &partition.content {
                        disk::Content::Filesystem(filesystem) => filesystem.create(&device),
                        disk::Content::Zfs(zfs) => zfs.create(&device),
                        disk::Content::Table(_) => {
                            panic!("nested partition tables aren't allowed");
                        }
                    };
                    create_cmds.push(create_content_cmds.join("\n"));
                    create_cmds.push("\n".to_string());
                }
            },
            disk::Content::Zfs(zfs) => {
                zfs.create(&disk_config.device);
            },
            disk::Content::Filesystem(filesystem) => {
                filesystem.create(&disk_config.device);
            },
        }
    }

    if let Some(zpools) = config.zpool {
        for (zpool_name, zpool_config) in zpools {
            create_cmds.push(zpool_config.create(&zpool_name).join("\n"));
        }
    }

    println!("{}", create_cmds.join("\n"));

    Ok(())
}
