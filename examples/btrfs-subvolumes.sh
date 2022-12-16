#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart ESP fat32 1MiB 128MiB

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 1 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.vfat \
    /dev/sdx1

parted -s /dev/sdx -- mkpart root 128MiB 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.btrfs /dev/sdx2
MNTPOINT=$(mktemp -d)
(
    mount /dev/sdx2 "$MNTPOINT"
    trap 'umount $MNTPOINT; rm -rf $MNTPOINT' EXIT
    btrfs subvolume create "$MNTPOINT"//home
    btrfs subvolume create "$MNTPOINT"//test
)
