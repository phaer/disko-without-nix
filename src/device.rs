use crate::{disk::Disk, lvm::LvmVg, mdadm::Mdadm, nodev::Nodev, zfs::Zpool};
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Devices {
    pub disk: HashMap<String, Disk>,
    pub mdadm: Option<HashMap<String, Mdadm>>,
    pub zpool: Option<HashMap<String, Zpool>>,
    pub lvm_vg: Option<HashMap<String, LvmVg>>,
    pub nodev: Option<HashMap<String, Nodev>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(try_from = "&str")]
pub struct DevicePath(pub String, pub Option<u8>);

impl fmt::Display for DevicePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self.1 {
            None => self.0.clone(),
            Some(index) => format!("{}{}", self.0, index),
        };
        write!(f, "{}", out)
    }
}

impl TryFrom<&str> for DevicePath {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref PATTERNS: [&'static str; 3] =
                [
                    r"(/dev/[vs]d[a-z]+)(.*)", // /dev/{s,v}da style
                    r"(/dev/disk/.+)-part(\d+)?", // /dev/disk /by-id/XXX-partY style
                    r"(/dev/disk/.+)?", // /dev/disk /by-id/XXX style
                    // TODO fix regex and tests
                    //r"(/dev/(?:nvme|md/|mmcblk)\d+)(?:p?(\d*))?" // /dev/nvme0n1p1 style
                ];
            static ref REGEXSET: RegexSet = RegexSet::new(&*PATTERNS)
                .expect("Regex does not compile");
            static ref REGEXES: Vec<Regex> =
                REGEXSET
                .patterns()
                .iter()
                .map(|pat| Regex::new(pat)
                     .expect("Regex does not compile"))
                .collect();
        }

        let captures: Vec<Captures> = REGEXSET
            .matches(value)
            .into_iter()
            .map(|match_idx| &REGEXES[match_idx])
            .map(|pat| pat.captures(value).unwrap())
            .collect();

        let capture = captures
            .get(0)
            .ok_or_else(|| anyhow!("device_path regex does not match"))?;
        //println!("Captured: {:#?}", capture);
        let path = capture
            .get(1)
            .ok_or(anyhow!("no path found"))?
            .as_str()
            .try_into()?;
        let index = capture.get(2).and_then(|m| m.as_str().parse::<u8>().ok());
        Ok(Self(path, index))
    }
}

#[test]
fn test_device_path_try_from() {
    assert_eq!(
        DevicePath("/dev/sdx".to_string(), None),
        "/dev/sdx".try_into().unwrap()
    );
    assert_eq!(
        DevicePath("/dev/sdx".to_string(), Some(1)),
        "/dev/sdx1".try_into().unwrap()
    );
    assert_eq!(
        DevicePath(
            "/dev/disk/by-id/scsi-0QEMU_QEMU_HARDDISK_19353493".to_string(),
            None
        ),
        "/dev/disk/by-id/scsi-0QEMU_QEMU_HARDDISK_19353493"
            .try_into()
            .unwrap()
    );
    assert_eq!(
        DevicePath(
            "/dev/disk/by-id/scsi-0QEMU_QEMU_HARDDISK_19353493".to_string(),
            Some(3)
        ),
        "/dev/disk/by-id/scsi-0QEMU_QEMU_HARDDISK_19353493-part3"
            .try_into()
            .unwrap()
    );
    //    assert_eq!(DevicePath("/dev/mmcblk0".to_string(), None), "/dev/mmcblk0".try_into().unwrap());
    //    assert_eq!(DevicePath("/dev/mmcblk0".to_string(), Some(10)), "/dev/mmcblk0p10".try_into().unwrap());
}

impl DevicePath {
    pub fn with_partition(&self, index: u8) -> Self {
        Self(self.0.clone(), Some(index))
    }

    pub fn with_path(&self, path: &str) -> Self {
        Self(path.to_string(), self.1)
    }

    pub fn is_partition(&self) -> bool {
        self.1.is_some()
    }
    //   pub fn deserialize<'de, D>(deserializer: D) -> Result<DevicePath, D::Error>
    //   where
    //       D: de::Deserializer<'de>,
    //   {
    //       let s: &str = de::Deserialize::deserialize(deserializer)?;
    //       serde_json::from_str(s).map_err(de::Error::custom)
    //   }
}
