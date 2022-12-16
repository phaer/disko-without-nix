#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart ESP fat32 0 64MiB

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 1 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.vfat \
    /dev/sdx1

parted -s /dev/sdx -- mkpart zfs 128MiB 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
ZFSDEVICES_zroot="${ZFSDEVICES_zroot:-}/dev/sdx2 "

parted -s /dev/sdy -- mklabel gpt
parted -s /dev/sdy -- mkpart zfs 128MiB 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
ZFSDEVICES_zroot="${ZFSDEVICES_zroot:-}/dev/sdy1 "

zpool create zroot \
    mirror \
    -O com.sun:auto-snapshot=false -O compression=lz4 \
    ${ZFSDEVICES_zroot}
zfs create zroot/zfs_fs \
    -o com.sun:auto-snapshot=true

zfs create zroot/zfs_legacy_fs \
    -o mountpoint=legacy

zfs create zroot/zfs_testvolume \
    -V 10M
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.ext4 \
    /dev/zvol/zroot/zfs_testvolume

zfs create zroot/zfs_unmounted_fs \
    -o mountpoint=none
