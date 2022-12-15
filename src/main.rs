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

    let mut create_script = String::new();

    for (_disk_name, disk_config) in &config.disk {
        match &disk_config.content {
            disk::DiskContent::Table(table) => {
                create_script.push_str(&format!(
                    "parted -s {} -- mklabel {}\n",
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

                    create_script.push_str(&format!(
                        "parted -s {} -- mkpart {} \n",
                        disk_config.device,
                        args.join(" ")
                    ));

                    create_script.push_str("# ensure /dev/disk/by-path/..-partN exists before continuing\n");
                    create_script.push_str("udevadm trigger --subsystem-match=block; udevadm settle\n");

                    if partition.bootable {
                        create_script.push_str(&format!(
                            "parted -s {} -- set {} boot on\n",
                            disk_config.device,
                            index
                        ));
                    }

                    for flag in &partition.flags {
                        create_script.push_str(&format!(
                            "parted -s {} -- set {} {} on\n",
                            disk_config.device,
                            index,
                            flag
                        ));
                    }

                    create_script.push_str("# ensure further operations can detect new partitions\n");
                    create_script.push_str("udevadm trigger --subsystem-match=block; udevadm settle\n");

                    let device = format!("{}{}", disk_config.device, index);  // TODO port deviceNumbering
                    match &partition.content {
                        partition::PartitionContent::Filesystem(filesystem) =>
                            create_script.push_str(&format!(
                                "mkfs.{} {} {}",
                                filesystem.format,
                                filesystem.extra_args.as_ref().map_or_else(|| "", |v| v.as_ref()),
                                device
                            )),
                        partition::PartitionContent::Zfs(zfs) =>
                            create_script.push_str(&format!(
                                "ZFSDEVICES_{}=\"${{ZFSDEVICES_{}:-}}{} \"",
                                zfs.pool,
                                zfs.pool,
                                device
                            )),
                    }
                    create_script.push_str("\n\n");
                }
            }
        }
    }

    if let Some(zpools) = config.zpool {
        for (zpool_name, zpool_config) in zpools {
            create_script.push_str(&format!(
                "zpool create {} {} {} {} ${{ZFSDEVICES_{}}}\n",
                zpool_name,
                zpool_config.mode,
                zfs::make_zfs_options(&zpool_config.options, "-o"),
                zfs::make_zfs_options(&zpool_config.root_fs_options, "-O"),
                zpool_name,
            ));

            for (dataset_name, dataset_config) in &zpool_config.datasets {
                match dataset_config {
                    zfs::ZfsDataset::Filesystem(filesystem) =>
                        create_script.push_str(&format!(
                            "zfs create {}/{} {}\n",
                            zpool_name,
                            dataset_name,
                            zfs::make_zfs_options(&filesystem.options, "-o")
                        )),
                    zfs::ZfsDataset::Volume(volume) => {
                        create_script.push_str(&format!(
                            "zfs create {}/{} {} -V {}\n",
                            zpool_name,
                            dataset_name,
                            zfs::make_zfs_options(&volume.options, "-o"),
                            volume.size
                        ));
                        create_script.push_str("udevadm trigger --subsystem-match=block; udevadm settle\n");
                        // TODO create volume contents
                    },
               }

            }
        }
    }

    println!("{}", create_script);

    Ok(())
}
