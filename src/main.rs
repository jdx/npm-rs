extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate digest;
extern crate reqwest;
extern crate sha1;
extern crate tar;
extern crate xx;

mod download;
mod package_lock;

use download::download_tarballs;
use package_lock::{readlock};

fn main() {
    let lock = readlock();
    download_tarballs(lock);
}
