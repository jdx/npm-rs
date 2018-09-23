extern crate tar;
extern crate xx;

use self::xx::hash;
use package_lock::PackageLock;
use std::io;
use std::path::Path;
// use tar::Archive;

pub fn verify_tarball(path: &Path, integrity: &String) -> Result<bool, io::Error> {
    let s: Vec<&str> = integrity.splitn(2, '-').collect();
    let method = s[0];
    let expected = xx::base64::decode_hex(s[1]).unwrap();
    let actual = match method {
        "sha1" => hash::file_sha1(path),
        "sha256" => hash::file_sha256(path),
        "sha512" => hash::file_sha512(path),
        _ => panic!("Unexpected method {}", method),
    }?;

    Ok(actual == expected)
}

pub fn extract_tarballs(lock: &PackageLock) {
    for (name, dep) in &lock.dependencies {
        verify_tarball(&dep.cache_path(), &dep.integrity).unwrap();
    }
}
