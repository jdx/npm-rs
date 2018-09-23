#[macro_use]
extern crate serde_derive;

mod download;
mod package_lock;

use download::download_tarballs;
use package_lock::readlock;

fn main() {
    let lock = readlock();
    download_tarballs(&lock);
}
