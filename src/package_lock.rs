extern crate serde_json;

use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLockDependency {
    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub requires: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLock {
    pub requires: bool,
    pub lockfile_version: u8,
    pub dependencies: HashMap<String, PackageLockDependency>,
}

pub fn readlock() -> PackageLock {
    let mut file = fs::File::open("fixtures/package-lock.json").expect("package-lock not found");
    let lock: PackageLock = serde_json::from_reader(&mut file).expect("invalid package-lock");
    println!("package-lock.json version: {}", lock.lockfile_version);
    lock
}
