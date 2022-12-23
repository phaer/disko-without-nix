use crate::device::{DevicePath, Devices};
use crate::disk::{Content, Disk, Table, TableFormat};
use crate::partition::Filesystem;
use crate::partition::Partition;
use crate::zfs::{make_zfs_options, ZfsDataset, ZfsFilesystem, ZfsPartition, ZfsVolume, Zpool};
use crate::swap::Swap;
use crate::btrfs::Btrfs;

impl Devices {
    pub fn create(&self) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        commands.push("#!/usr/bin/env bash\nset -efux".to_string());

        for (_disk_name, disk) in &self.disk {
            commands.append(&mut disk.create())
        }

        if let Some(zpools) = &self.zpool {
            for (zpool_name, zpool_config) in zpools {
                commands.append(&mut zpool_config.create(&zpool_name));
            }
        }
        commands
    }
}

impl Disk {
    pub fn create(&self) -> Vec<String> {
        self.content.create(&self.device)
    }
}

impl Table {
    pub fn create(&self, device_path: &DevicePath) -> Vec<String> {
        if device_path.is_partition() {
            panic!("Partition tables can't be nested")
        }

        let mut commands: Vec<String> = Vec::new();
        commands.push(format!(
            "parted -s {} -- mklabel {}",
            device_path,
            self.format.as_ref(),
        ));

        let mut index = 0;
        for partition in &self.partitions {
            index += 1;
            commands.append(&mut partition.create(device_path, &self.format, index))
        }
        commands
    }
}

impl Partition {
    pub fn create(
        &self,
        device_path: &DevicePath,
        table_format: &TableFormat,
        index: u8,
    ) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let fs_type = self.fs_type.as_ref().map_or_else(|| "", |v| v.as_ref());
        let mut args = Vec::new();
        match table_format {
            TableFormat::Gpt => args.push(self.name.as_str()),
            TableFormat::Msdos => {
                args.push(self.part_type.as_ref());
                args.push(fs_type);
            }
        }
        args.append(&mut vec![fs_type, &self.start, &self.end]);
        args.retain(|&s| s != "");

        commands.push(format!(
            "parted -s {} -- mkpart {}\n",
            device_path,
            args.join(" ")
        ));

        commands.push("# ensure /dev/disk/by-path/..-partN exists before continuing".to_string());
        commands.push("udevadm trigger --subsystem-match=block\nudevadm settle".to_string());

        if self.bootable {
            commands.push(format!(
                "parted -s {} -- set {} boot on",
                device_path, index
            ));
        }

        for flag in &self.flags {
            commands.push(format!(
                "\nparted -s {} -- set {} {} on",
                device_path, index, flag
            ));
        }
        commands.push("".to_string());

        commands.push("# ensure further operations can detect new partitions".to_string());
        commands.push("udevadm trigger --subsystem-match=block\nudevadm settle".to_string());

        let partition_path = device_path.with_partition(index);
        let create_content_cmds = &self.content.create(&partition_path);
        commands.push(create_content_cmds.join("\n"));
        commands
    }
}

impl Content {
    pub fn create(&self, device_path: &DevicePath) -> Vec<String> {
        match self {
            Content::Table(table) => table.create(device_path),
            Content::Zfs(zfs) => zfs.create(device_path),
            Content::Filesystem(filesystem) => filesystem.create(device_path),
            Content::Swap(swap) => swap.create(device_path),
            Content::Btrfs(btrfs) => btrfs.create(device_path),
            Content::None => Vec::new(),
            Content::Mdraid(_)
                | Content::Luks(_)
                | Content::LvmPv(_) => {
                    eprintln!("Warning: {:?} is not implemented yet, PRs welcome!", self);
                    vec![format!("# {:?} is not implemented yet", self)]
                },
        }
    }
}

impl Filesystem {
    pub fn create(&self, device: &DevicePath) -> Vec<String> {
        vec![format!(
            "mkfs.{} \\\n   {} {}\n",
            &self.format,
            &self.extra_args.as_ref().map_or_else(|| "", |v| v.as_ref()),
            device
        )]
    }
}

impl Swap {
    pub fn create(&self, device: &DevicePath) -> Vec<String> {
        vec![format!(
            "mkswap {}\n",
            device
        )]
    }
}

impl Btrfs {
    pub fn create(&self, device: &DevicePath) -> Vec<String> {
        vec![format!(
            "mkfs.btrfs {device}
MNTPOINT=$(mktemp -d)
(
    mount {device} \"$MNTPOINT\"
    trap 'umount $MNTPOINT; rm -rf $MNTPOINT' EXIT
    btrfs subvolume create \"$MNTPOINT\"//home
    btrfs subvolume create \"$MNTPOINT\"//test
)",
            device=device
        )]
    }
}


impl ZfsPartition {
    pub fn create(&self, device: &DevicePath) -> Vec<String> {
        let mut commands = Vec::new();
        commands.push(format!(
            "ZFSDEVICES_{}=\"${{ZFSDEVICES_{}:-}}{} \"",
            &self.pool, &self.pool, device
        ));
        if device.is_partition() {
            commands.push(String::new())
        }
        commands
    }
}

impl Zpool {
    pub fn create(&self, zpool_name: &str) -> Vec<String> {
        let mut commands = Vec::new();
        let mode = if self.mode != "" { self.mode.clone() + " \\\n    " } else { self.mode.clone() };
        commands.push(format!(
            "zpool create {zpool} \\\n    {mode}{options}{root_fs_options} \\\n    ${{ZFSDEVICES_{zpool}}}",
            zpool=zpool_name,
            mode=mode,
            options=make_zfs_options(&self.options, "-o"),
            root_fs_options=make_zfs_options(&self.root_fs_options, "-O"),
        ));

        for (dataset_name, dataset_config) in &self.datasets {
            commands.append(&mut match dataset_config {
                ZfsDataset::Filesystem(filesystem) => filesystem.create(zpool_name, dataset_name),
                ZfsDataset::Volume(volume) => volume.create(zpool_name, dataset_name),
            })
        }
        commands
    }
}

impl ZfsFilesystem {
    pub fn create(&self, zpool_name: &str, dataset_name: &str) -> Vec<String> {
        vec![
            format!(
                "zfs create {}/{} \\\n    {}",
                zpool_name,
                dataset_name,
                make_zfs_options(&self.options, "-o")
            ),
            String::from(""),
        ]
    }
}

impl ZfsVolume {
    pub fn create(&self, zpool_name: &str, dataset_name: &str) -> Vec<String> {
        let mut commands = Vec::new();
        let volume_path = DevicePath::try_from(vec!["/dev/zvol", zpool_name, dataset_name].join("/").as_ref()).unwrap();
        commands.push(format!(
                "zfs create {}/{} \\\n    {}-V {}",
                zpool_name,
                dataset_name,
                make_zfs_options(&self.options, "-o"),
                self.size
        ));
        commands.push(String::from("udevadm trigger --subsystem-match=block\nudevadm settle"));
        commands.append(&mut self.content.create(&volume_path));
        commands
    }
}
