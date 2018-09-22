extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tar;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fs;
use std::io;
// use std::io::{self, Write};
use std::path::Path;
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
    let mut file = fs::File::open("fixtures/package-lock.json")?;
    let lock: PackageLock = serde_json::from_reader(&mut file)?;
    println!("package-lock.json version: {}", lock.lockfile_version);
    Ok(lock)
}

fn download_tarballs(lock: PackageLock) -> Result<(), io::Error> {
    for (name, dep) in lock.dependencies.iter() {
        println!("{} {}", name, dep.resolved);
        download_file(&dep.resolved, &format!("tmp/{}.tgz", name))?;
    }
    Ok(())
}

fn download_file(url: &String, to: &String) -> Result<(), io::Error> {
    println!("downloading {} to {}", url, to);
    let mut response = reqwest::get(url).expect("http failed");
    response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .expect("download failed");
    let dir = Path::new(to).parent().unwrap();
    fs::create_dir_all(dir)?;
    io::copy(&mut response, &mut fs::File::create(to)?)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let lock = readlock()?;
    download_tarballs(lock)?;
    Ok(())
}
