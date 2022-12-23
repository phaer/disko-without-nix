use disko::Devices;
use std::path::Path;
use anyhow::Result;

fn create_script(path: &Path) -> Result<String> {
    let config_file = std::fs::read_to_string(path)?;
    let devices: Devices = serde_json::from_str(&config_file)?;
    Ok(devices.create().join("\n"))
}

#[test]
fn generate_create_scripts() -> Result<()> {
    let examples_path = &Path::new("./examples");
    for entry in examples_path.read_dir()? {
        let path = entry?.path();
        let extension = path.extension().and_then(|v| v.to_str());
        if Some("json") != extension {
            continue;
        }
        create_script(&path)?;
    }
    Ok(())
}

#[test]
fn verify_create_scripts() -> Result<()> {
    let examples_path = &Path::new("./examples");
    for entry in examples_path.read_dir()? {
        let path = entry?.path();
        let name: &str = path.file_stem().and_then(|v|v.to_str()).unwrap();
        let extension = path.extension().and_then(|v| v.to_str());
        if Some("json") != extension {
            continue;
        }
        insta::assert_display_snapshot!(name, create_script(&path)?);
    }
    Ok(())
}
