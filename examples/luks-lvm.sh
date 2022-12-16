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

parted -s /dev/sdx -- mkpart luks  100MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
cryptsetup -q luksFormat /dev/sdx2 /tmp/secret.key 
cryptsetup luksOpen /dev/sdx2 crypted --key-file /tmp/secret.key
pvcreate /dev/mapper/crypted
LVMDEVICES_pool="${LVMDEVICES_pool:-}/dev/mapper/crypted "



vgcreate pool $LVMDEVICES_pool
lvcreate \
  -L 10M \
  -n home \
   \
   \
  pool
mkfs.ext4 \
   \
  /dev/pool/home

lvcreate \
  -L 10M \
  -n raw \
   \
   \
  pool

lvcreate \
  -L 100M \
  -n root \
   \
   \
  pool
mkfs.ext4 \
   \
  /dev/pool/root




