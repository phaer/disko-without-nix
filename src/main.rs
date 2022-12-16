use anyhow::Result;

mod create;
mod device;
mod disk;
mod lvm;
mod mdadm;
mod nodev;
mod partition;
mod zfs;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Please pass a json file as the first argument."),
    };
    let config_file = std::fs::read_to_string(filename)?;
    let devices: device::Devices = serde_json::from_str(&config_file)?;

    let mut create_cmds: Vec<String> = Vec::new();

    for (_disk_name, disk) in &devices.disk {
        create_cmds.append(&mut disk.create())
    }

    if let Some(zpools) = devices.zpool {
        for (zpool_name, zpool_config) in zpools {
            create_cmds.append(&mut zpool_config.create(&zpool_name));
        }
    }

    println!("{}", create_cmds.join("\n"));

    Ok(())
}
