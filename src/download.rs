extern crate digest;
extern crate reqwest;
extern crate sha1;
extern crate tar;
extern crate xx;

use package_lock::PackageLock;
use std::io;
use std::path::Path;

pub fn download_tarballs(lock: &PackageLock) {
    for (name, dep) in &lock.dependencies {
        let file = Path::new("tmp")
            .join(name)
            .join(format!("{}-{}.tgz", name, dep.version));
        let s: Vec<&str> = dep.integrity.splitn(2, '-').collect();
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
                    println!(
                        "hash fail: expected hash of {:?}:{} to be {}",
                        file, actual, expected
                    );
                    xx::http::download(&dep.resolved, &file).unwrap();
                }
            }
            None => {
                println!("file not found: {:?}", file);
                xx::http::download(&dep.resolved, &file).unwrap();
            }
        };
    }
}
