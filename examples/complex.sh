#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart ESP fat32 1MiB 128MiB


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdx -- set 1 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.vfat \
   \
  /dev/sdx1


parted -s /dev/sdy -- mklabel gpt
parted -s /dev/sdy -- mkpart luks  1M 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
cryptsetup -q luksFormat /dev/sdy1 /tmp/secret.key 
cryptsetup luksOpen /dev/sdy1 crypted1 --key-file /tmp/secret.key
pvcreate /dev/mapper/crypted1
LVMDEVICES_pool="${LVMDEVICES_pool:-}/dev/mapper/crypted1 "



parted -s /dev/sdz -- mklabel gpt
parted -s /dev/sdz -- mkpart luks  1M 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
cryptsetup -q luksFormat /dev/sdz1 /tmp/secret.key 
cryptsetup luksOpen /dev/sdz1 crypted2 --key-file /tmp/secret.key
pvcreate /dev/mapper/crypted2
LVMDEVICES_pool="${LVMDEVICES_pool:-}/dev/mapper/crypted2 "



vgcreate pool $LVMDEVICES_pool
lvcreate \
  -L 30M \
  -n raid1 \
  --type=raid0 \
   \
  pool
RAIDDEVICES_N_raid1=$((${RAIDDEVICES_N_raid1:-0}+1))
RAIDDEVICES_raid1="${RAIDDEVICES_raid1:-}/dev/pool/raid1 "

lvcreate \
  -L 30M \
  -n raid2 \
  --type=raid0 \
   \
  pool
RAIDDEVICES_N_raid1=$((${RAIDDEVICES_N_raid1:-0}+1))
RAIDDEVICES_raid1="${RAIDDEVICES_raid1:-}/dev/pool/raid2 "

lvcreate \
  -L 10M \
  -n root \
  --type=mirror \
   \
  pool
mkfs.ext4 \
   \
  /dev/pool/root

lvcreate \
  -L 128M \
  -n zfs1 \
  --type=raid0 \
   \
  pool
ZFSDEVICES_zroot="${ZFSDEVICES_zroot:-}/dev/pool/zfs1 "

lvcreate \
  -L 128M \
  -n zfs2 \
  --type=raid0 \
   \
  pool
ZFSDEVICES_zroot="${ZFSDEVICES_zroot:-}/dev/pool/zfs2 "


echo 'y' | mdadm --create /dev/md/raid1 \
  --level=1 \
  --raid-devices=${RAIDDEVICES_N_raid1} \
  --metadata=default \
  --homehost=any \
  ${RAIDDEVICES_raid1}
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/md/raid1 -- mklabel gpt
parted -s /dev/md/raid1 -- mkpart bla  1MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/md/raid1p1



zpool create zroot \
  mirror \
   \
  -O com.sun:auto-snapshot=false -O compression=lz4 \
  ${ZFSDEVICES_zroot}
zfs create zroot/zfs_fs \
  -o com.sun:auto-snapshot=true \
  

zfs create zroot/zfs_legacy_fs \
  -o mountpoint=legacy \
  

zfs create zroot/zfs_testvolume \
   \
  -V 10M
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/zvol/zroot/zfs_testvolume


zfs create zroot/zfs_unmounted_fs \
  -o mountpoint=none \
  




