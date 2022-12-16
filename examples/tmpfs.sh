#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart ESP 1MiB 100MiB

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 1 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.vfat \
    /dev/sdx1

parted -s /dev/sdx -- mkpart root 100MiB 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 2 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.ext4 \
    /dev/sdx2
