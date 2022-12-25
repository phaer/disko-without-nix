use disko::Devices;
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use glob::glob;

fn get_examples(pattern: &str) -> Result<impl Iterator<Item = (String, PathBuf)>> {
    Ok(glob(pattern)?
       .filter_map(|entry| match entry {
           Ok(entry) => {
               let name: String = entry.file_stem().unwrap().to_string_lossy().to_string();
               match entry.extension().and_then(|v|v.to_str()) {
                   Some("json") => Some((name, entry)),
                   _ => None
               }
           }
           Err(_) => None
       }))
}

fn create_script(path: &Path) -> Result<String> {
    let config_file = std::fs::read_to_string(path)?;
    let devices: Devices = serde_json::from_str(&config_file)?;
    Ok(devices.create().join("\n").trim().to_string())
}

fn mount_script(path: &Path) -> Result<String> {
    let config_file = std::fs::read_to_string(path)?;
    let devices: Devices = serde_json::from_str(&config_file)?;
    Ok(devices.mount().join("\n").trim().to_string())
}

#[test]
fn generate_create_scripts() -> Result<()> {
    for (_name, path) in get_examples("./examples/*.json")? {
        create_script(&path)?;
    }
    Ok(())
}

#[test]
fn verify_create_scripts() -> Result<()> {
    for (name, path) in get_examples("./examples/*.json")? {
        insta::assert_display_snapshot!(format!("create-{}", name), create_script(&path)?);
    }
    Ok(())

}

#[test]
fn generate_mount_scripts() -> Result<()> {
    for (_name, path) in get_examples("./examples/*.json")? {
        mount_script(&path)?;
    }
    Ok(())
}

#[test]
fn verify_mount_scripts() -> Result<()> {
    for (name, path) in get_examples("./examples/*.json")? {
        insta::assert_display_snapshot!(format!("mount-{}", name), mount_script(&path)?);
    }
    Ok(())
}
