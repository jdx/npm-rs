#[macro_use]
extern crate serde_derive;

mod package_lock;

use package_lock::PackageLock;
use std::fs;

fn main() {
    let root = fs::canonicalize("fixtures").unwrap();
    let lock = PackageLock::new(&root);
    lock.install();
}
