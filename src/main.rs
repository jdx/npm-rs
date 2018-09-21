
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageLock {
    requires: bool,
    lockfile_version: u8,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("fixtures/package-lock.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lock: PackageLock = serde_json::from_str(&mut contents)?;
    println!("version {}", lock.lockfile_version);
    Ok(())
}
