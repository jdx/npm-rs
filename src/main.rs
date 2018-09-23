extern crate digest;
extern crate xx;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha1;
extern crate tar;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

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
        let file = Path::new("tmp")
            .join(name)
            .join(format!("{}-{}.tgz", name, dep.version));
        let s: Vec<&str> = dep.integrity.splitn(2, "-").collect();
        let method = s[0];
        let expected = xx::base64::decode_hex(s[1]).unwrap();
        let get_sha = |hasher: fn(&str) -> Result<String, io::Error>| -> Option<String> {
            match hasher(&file.to_str().unwrap()) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        };
        let actual = match method {
            "sha1" => get_sha(xx::hash::file_sha1),
            "sha256" => get_sha(xx::hash::file_sha256),
            "sha512" => get_sha(xx::hash::file_sha512),
            _ => panic!("Unexpected method {}", method),
        };
        match actual {
            Some(actual) => {
                if actual != expected {
                    println!("{:?}", actual);
                    println!("{:?}", expected);
                    println!("hash fail: expected hash of {:?}:{} to be {}", file, actual, expected);
                    download_file(&dep.resolved, &file)?;
                }
            },
            None => {
                println!("file not found: {:?}", file);
                download_file(&dep.resolved, &file)?;
            }
        }
    }
    Ok(())
}

fn download_file(url: &String, to: &Path) -> Result<(), io::Error> {
    println!("downloading {} to {}", url, to.to_str().unwrap());
    let mut response = reqwest::get(url).expect("http failed");
    response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .expect("download failed");
    fs::create_dir_all(to.parent().unwrap())?;
    io::copy(&mut response, &mut fs::File::create(to)?)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let lock = readlock()?;
    download_tarballs(lock)?;
    Ok(())
}
