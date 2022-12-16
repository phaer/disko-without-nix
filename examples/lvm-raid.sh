#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart boot fat32 0 100M

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdx -- set 1 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
RAIDDEVICES_N_boot=$((${RAIDDEVICES_N_boot:-0} + 1))
RAIDDEVICES_boot="${RAIDDEVICES_boot:-}/dev/sdx1 "

parted -s /dev/sdx -- mkpart primary 100M 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
pvcreate /dev/sdx2
LVMDEVICES_pool="${LVMDEVICES_pool:-}/dev/sdx2 "

parted -s /dev/sdy -- mklabel gpt
parted -s /dev/sdy -- mkpart boot fat32 0 100M

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle
parted -s /dev/sdy -- set 1 boot on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
RAIDDEVICES_N_boot=$((${RAIDDEVICES_N_boot:-0} + 1))
RAIDDEVICES_boot="${RAIDDEVICES_boot:-}/dev/sdy1 "

parted -s /dev/sdy -- mkpart primary 100M 100%

# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block
udevadm settle

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block
udevadm settle
pvcreate /dev/sdy2
LVMDEVICES_pool="${LVMDEVICES_pool:-}/dev/sdy2 "

vgcreate pool $LVMDEVICES_pool
lvcreate \
    -L 10M \
    -n home \
    --type=raid0 \
    pool
mkfs.ext4 \
    /dev/pool/home

lvcreate \
    -L 100M \
    -n root \
    --type=mirror \
    pool
mkfs.ext4 \
    /dev/pool/root

echo 'y' | mdadm --create /dev/md/boot \
    --level=1 \
    --raid-devices=${RAIDDEVICES_N_boot} \
    --metadata=1.0 \
    --homehost=any \
    ${RAIDDEVICES_boot}
udevadm trigger --subsystem-match=block
udevadm settle
mkfs.vfat \
    /dev/md/boot
