#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart boot  0 1M


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle

parted -s /dev/sdx -- set 1 bios_grub on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle

parted -s /dev/sdx -- mkpart ESP fat32 1MiB 128MiB


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdx -- set 2 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
RAIDDEVICES_N_boot=$((${RAIDDEVICES_N_boot:-0}+1))
RAIDDEVICES_boot="${RAIDDEVICES_boot:-}/dev/sdx2 "

parted -s /dev/sdx -- mkpart mdadm  128MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
RAIDDEVICES_N_raid1=$((${RAIDDEVICES_N_raid1:-0}+1))
RAIDDEVICES_raid1="${RAIDDEVICES_raid1:-}/dev/sdx3 "


parted -s /dev/sdy -- mklabel gpt
parted -s /dev/sdy -- mkpart boot  0 1M


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle

parted -s /dev/sdy -- set 1 bios_grub on

# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle

parted -s /dev/sdy -- mkpart ESP fat32 1MiB 128MiB


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdy -- set 2 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
RAIDDEVICES_N_boot=$((${RAIDDEVICES_N_boot:-0}+1))
RAIDDEVICES_boot="${RAIDDEVICES_boot:-}/dev/sdy2 "

parted -s /dev/sdy -- mkpart mdadm  128MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
RAIDDEVICES_N_raid1=$((${RAIDDEVICES_N_raid1:-0}+1))
RAIDDEVICES_raid1="${RAIDDEVICES_raid1:-}/dev/sdy3 "


echo 'y' | mdadm --create /dev/md/boot \
  --level=1 \
  --raid-devices=${RAIDDEVICES_N_boot} \
  --metadata=1.0 \
  --homehost=any \
  ${RAIDDEVICES_boot}
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.vfat \
   \
  /dev/md/boot

echo 'y' | mdadm --create /dev/md/raid1 \
  --level=1 \
  --raid-devices=${RAIDDEVICES_N_raid1} \
  --metadata=default \
  --homehost=any \
  ${RAIDDEVICES_raid1}
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/md/raid1 -- mklabel gpt
parted -s /dev/md/raid1 -- mkpart primary  1MiB 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/md/raid1p1





