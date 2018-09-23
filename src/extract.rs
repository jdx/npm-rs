extern crate flate2;
extern crate tar;
extern crate xx;

use self::flate2::read::GzDecoder;
use self::tar::Archive;
use self::xx::hash;
use package_lock::PackageLock;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub fn verify_tarball(path: &Path, integrity: &String, must: bool) -> Result<bool, io::Error> {
    let s: Vec<&str> = integrity.splitn(2, '-').collect();
    let method = s[0];
    let expected = xx::base64::decode_hex(s[1]).unwrap();
    let actual = match method {
        "sha1" => hash::file_sha1(path),
        "sha256" => hash::file_sha256(path),
        "sha512" => hash::file_sha512(path),
        _ => panic!("Unexpected method {}", method),
    }?;
    let m = actual == expected;
    if !m && must {
        panic!(
            "hash mismatch path: {:?}\nexpected: {}\nactual: {}",
            path, expected, actual
        );
    }

    Ok(m)
}

pub fn extract_tarballs<P: AsRef<Path>>(lock: &PackageLock, _root: P) {
    let root = _root.as_ref();
    for (name, dep) in &lock.dependencies {
        let tarball = &dep.cache_path();
        verify_tarball(tarball, &dep.integrity, true).unwrap();
        println!("unpacking {:?}", tarball);
        let mut file = GzDecoder::new(File::open(tarball).unwrap());
        let mut archive = Archive::new(file);
        let base = root.join("node_modules");
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let path = get_real_path(name, file.path().unwrap().as_ref(), &base);
            println!("{:?}", path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut output = File::create(&path).unwrap();
            io::copy(&mut file, &mut output).unwrap();
        }
    }
}

fn get_real_path(name: &str, path: &Path, root: &PathBuf) -> PathBuf {
    let orig_path = path;
    let path = root
        .join(name)
        .join(orig_path.strip_prefix("package").unwrap());
    if !path.starts_with(root) {
        panic!("invalid tarball");
    }

    path
}
