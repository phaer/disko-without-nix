use crate::device::{DevicePath, Devices};
use crate::disk::{Content, Disk, Table, TableFormat};
use crate::partition::Filesystem;
use crate::partition::Partition;
use crate::zfs::{make_zfs_options, ZfsDataset, ZfsFilesystem, ZfsPartition, ZfsVolume, Zpool};

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

        commands.push(format!(
            "parted -s {} -- mkpart {} ",
            device_path,
            args.join(" ")
        ));

        commands.push("# ensure /dev/disk/by-path/..-partN exists before continuing".to_string());
        commands.push("udevadm trigger --subsystem-match=block; udevadm settle".to_string());

        if self.bootable {
            commands.push(format!(
                "parted -s {} -- set {} boot on",
                device_path, index
            ));
        }

        for flag in &self.flags {
            commands.push(format!(
                "parted -s {} -- set {} {} on",
                device_path, index, flag
            ));
        }

        commands.push("# ensure further operations can detect new partitions".to_string());
        commands.push("udevadm trigger --subsystem-match=block; udevadm settle".to_string());

        let partition_path = device_path.with_partition(index);
        let create_content_cmds = &self.content.create(&partition_path);
        commands.push(create_content_cmds.join("\n"));
        commands.push("\n".to_string());
        commands
    }
}

impl Content {
    pub fn create(&self, device_path: &DevicePath) -> Vec<String> {
        match self {
            Content::Table(table) => table.create(device_path),
            Content::Zfs(zfs) => zfs.create(device_path),
            Content::Filesystem(filesystem) => filesystem.create(device_path),
            Content::None => Vec::new(),
            Content::Mdraid(_)
                | Content::Btrfs(_)
                | Content::Swap(_)
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
            "mkfs.{} {} {}",
            &self.format,
            &self.extra_args.as_ref().map_or_else(|| "", |v| v.as_ref()),
            device
        )]
    }
}

impl ZfsPartition {
    pub fn create(&self, device: &DevicePath) -> Vec<String> {
        vec![format!(
            "ZFSDEVICES_{}=\"${{ZFSDEVICES_{}:-}}{} \"",
            &self.pool, &self.pool, device
        )]
    }
}

impl Zpool {
    pub fn create(&self, zpool_name: &str) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        commands.push(format!(
            "zpool create {} {} {} {} ${{ZFSDEVICES_{}}}",
            zpool_name,
            self.mode,
            make_zfs_options(&self.options, "-o"),
            make_zfs_options(&self.root_fs_options, "-O"),
            zpool_name,
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
        vec![format!(
            "zfs create {}/{} {}",
            zpool_name,
            dataset_name,
            make_zfs_options(&self.options, "-o")
        )]
    }
}

impl ZfsVolume {
    pub fn create(&self, zpool_name: &str, dataset_name: &str) -> Vec<String> {
        vec![
            format!(
                "zfs create {}/{} {} -V {}",
                zpool_name,
                dataset_name,
                make_zfs_options(&self.options, "-o"),
                self.size
            ),
            String::from("udevadm trigger --subsystem-match=block; udevadm settle"), // TODO create volume contents
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use anyhow::Result;

    fn create_script(path: &Path) -> Result<String> {
        let config_file = std::fs::read_to_string(path)?;
        let devices: Devices = serde_json::from_str(&config_file)?;
        Ok(devices.create().join("\n"))
    }

    #[test]
    fn generate_create_scripts() -> Result<()> {
        let examples_path = &Path::new("./examples");
        for entry in examples_path.read_dir()? {
            let path = entry?.path();
            let name: &str = path.file_stem().and_then(|v|v.to_str()).unwrap();
            let extension = path.extension().and_then(|v| v.to_str());
            if Some("json") != extension {
                continue;
            }
            create_script(&path)?;
        }
        Ok(())
    }

    #[test]
    fn verify_create_scripts() -> Result<()> {
        let examples_path = &Path::new("./examples");
        for entry in examples_path.read_dir()? {
            let path = entry?.path();
            let name: &str = path.file_stem().and_then(|v|v.to_str()).unwrap();
            let extension = path.extension().and_then(|v| v.to_str());
            if Some("json") != extension {
                continue;
            }
            insta::assert_display_snapshot!(name, create_script(&path)?);
        }
        Ok(())
    }
}
