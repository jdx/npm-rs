#[macro_use]
extern crate serde_derive;

mod extract;
mod download;
mod package_lock;

use download::download_tarballs;
use package_lock::PackageLock;
use extract::extract_tarballs;
use std::fs;

fn main() {
    let root = fs::canonicalize("fixtures").unwrap();
    let lock = PackageLock::new(&root);
    download_tarballs(&lock);
    extract_tarballs(&lock, &root);
}
