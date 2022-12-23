# disko-without-nix

A small experimental rust program, to build shell scripts from 
[disko´s](https://github.com/nix-community/disko) nix expressions, converted 
to json. The idea is to pre-generate the json files for your disk layouts on 
a system with nix installed, while still allowing to parameterize things like
the specific disks to use at run-time, even if nix and nixpkgs aren't 
available yet. I.e. in space-constrained installers like
[nix-dabei](https://github.com/dep-sys/nix-dabei)

It isn't finished yet, but work in progress.

# Snapshot / Integration tests

To ensure compatibility upstream disko, we use [insta.rs](https://insta.rs/)
for snapshot testing, while pre-generating snapshots from diskos examples.

* (optionally) `nix run .#updateExamples` to evaluate all nix examples and serialize
config to JSON via `builtins.toJSON` and scripts via `disko.lib.createScriptNoDeps`
to `./examples`, overwriting the existing ones.

* (optionally) `nix run .#updateSnapshots` to pre-generate insta.rs snapshots from diskos
  examples to `./tests/snapshots`.

* `cargo insta test` to compare our output to the snapshots
* `cargo insta review` to see diffs between old, pre-generated snapshots and our output.

# Checklist

- [x] types for disks
- [x] types for partitions 
- [x] types for zfs
- [ ] types for mdadm
- [ ] types for btrfs
- [ ] types for swap
- [ ] types for luks
- [ ] types for lvm
- [ ] types for nodev
- [x] create script
- [ ] mount script
- [ ] allow placeholders/variables for disk paths
- [ ] output nixos configuration values as json?
- [ ] linting for disk configurations?
