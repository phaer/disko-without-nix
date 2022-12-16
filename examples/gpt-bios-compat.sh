#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart boot 0 1M

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

parted -s /dev/sdx -- set 1 bios_grub on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle

parted -s /dev/sdx -- mkpart root 1M 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 2 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.ext4 \
    /dev/sdx2
