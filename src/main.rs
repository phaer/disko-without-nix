use anyhow::Result;

mod create;
mod device;
mod disk;
mod lvm;
mod mdadm;
mod nodev;
mod partition;
mod zfs;
mod btrfs;
mod swap;
mod luks;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Please pass a json file as the first argument."),
    };
    let config_file = std::fs::read_to_string(filename)?;
    let devices: device::Devices = serde_json::from_str(&config_file)?;

    let create_commands = devices.create();

    println!("{}", create_commands.join("\n"));

    Ok(())
}
