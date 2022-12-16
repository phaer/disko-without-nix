#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart ESP  1MiB 100MiB


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdx -- set 1 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.vfat \
   \
  /dev/sdx1

parted -s /dev/sdx -- mkpart primary  100MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdx -- set 2 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/sdx2


ZFSDEVICES_zroot="${ZFSDEVICES_zroot:-}/dev/sdy "
zpool create zroot \
   \
   \
  -O mountpoint=none \
  ${ZFSDEVICES_zroot}
zfs create zroot/root \
  -o mountpoint=none \
  

zfs create zroot/root/zfs_fs \
  -o com.sun:auto-snapshot=true -o mountpoint=/zfs_fs \
  




