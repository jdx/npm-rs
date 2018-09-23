#[macro_use]
extern crate serde_derive;

mod extract;
mod download;
mod package_lock;

use download::download_tarballs;
use package_lock::PackageLock;
use extract::extract_tarballs;

fn main() {
    let lock = PackageLock::new("fixtures");
    download_tarballs(&lock);
    extract_tarballs(&lock);
}
