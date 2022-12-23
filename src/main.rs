use anyhow::Result;
use disko::Devices;


fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Please pass a json file as the first argument."),
    };
    let config_file = std::fs::read_to_string(filename)?;
    let devices: Devices = serde_json::from_str(&config_file)?;

    let create_commands = devices.create();

    println!("{}", create_commands.join("\n"));

    Ok(())
}
