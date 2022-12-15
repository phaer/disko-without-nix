# disko-without-nix

A small experimental rust program, to build shell scripts from 
[disko´s](https://github.com/nix-community/disko) nix expressions, converted 
to json. The idea is to pre-generate the json files for your disk layouts on 
a system with nix installed, while still allowing to parameterize things like
the specific disks to use at run-time, even if nix and nixpkgs aren't 
available yet. I.e. in space-constrained installers like
[nix-dabei](https://github.com/dep-sys/nix-dabei)

It isn't finished yet, but work in progress.

- [x] types for disks
- [x] types for partitions 
- [x] types for zfs
- [ ] types for mdadm
- [ ] types for lvm
- [ ] types for nodev
- [ ] script generation
