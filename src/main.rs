extern crate hyper;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use hyper::rt::{self, Future, Stream};
use hyper::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

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

fn readlock() -> std::io::Result<()> {
    let mut file = File::open("fixtures/package-lock.json")?;
    let lock: PackageLock = serde_json::from_reader(&mut file)?;
    println!("version {}", lock.lockfile_version);
    let dependency = lock.dependencies.get("ansi-styles").unwrap();
    println!(
        "tarball {}@{}: {}",
        "ansi-styles", dependency.version, dependency.resolved
    );
    Ok(())
}

fn testhttp() -> impl Future<Item = (), Error = ()> {
    let uri = "http://httpbin.org/ip".parse().unwrap();
    let client = Client::new();

    client
        .get(uri)
        .and_then(|res| {
            res.into_body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

fn main() -> std::io::Result<()> {
    readlock()?;
    rt::run(testhttp());
    Ok(())
}
