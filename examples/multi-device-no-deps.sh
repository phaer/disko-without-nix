#!/usr/bin/env bash
set -efux
parted -s /dev/sdx -- mklabel gpt
parted -s /dev/sdx -- mkpart nix  0% 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdx -- set 1 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/sdx1


parted -s /dev/sdy -- mklabel gpt
parted -s /dev/sdy -- mkpart root  0% 100%


# ensure /dev/disk/by-path/..-partN exists before continuing
udevadm trigger --subsystem-match=block; udevadm settle
parted -s /dev/sdy -- set 1 boot on


# ensure further operations can detect new partitions
udevadm trigger --subsystem-match=block; udevadm settle
mkfs.ext4 \
   \
  /dev/sdy1




