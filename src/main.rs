extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tar;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::Error;
// use std::io::{self, Write};
// use std::path::Path;
// use tar::Archive;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageLockDependency {
    version: String,
    resolved: String,
    integrity: String,
    requires: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageLock {
    requires: bool,
    lockfile_version: u8,
    dependencies: HashMap<String, PackageLockDependency>,
}

fn readlock() -> std::io::Result<PackageLock> {
    let mut file = File::open("fixtures/package-lock.json")?;
    let lock: PackageLock = serde_json::from_reader(&mut file)?;
    println!("package-lock.json version: {}", lock.lockfile_version);
    Ok(lock)
}

fn download_file(url: &String, to: &String) -> Result<(), Error> {
    println!("downloading {} to {}", url, to);
    let mut response = reqwest::get(url).expect("http failed");
    response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .expect("download failed");
    copy(&mut response, &mut File::create(to)?)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let lock = readlock()?;
    let dependency = lock.dependencies.get("ansi-styles").unwrap();
    println!(
        "tarball: {}@{}: {}",
        "ansi-styles", dependency.version, dependency.resolved
    );
    download_file(&dependency.resolved, &String::from("tarball.tgz"))?;
    Ok(())
}
