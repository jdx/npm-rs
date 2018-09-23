extern crate xx;

use extract::verify_tarball;
use package_lock::PackageLock;

pub fn download_tarballs(lock: &PackageLock) {
    for (_, dep) in &lock.dependencies {
        let file = dep.cache_path();
        match verify_tarball(&file, &dep.integrity, false) {
            Ok(verified) => {
                if !verified {
                    println!("hash fail: {:?}", file);
                    xx::http::download(&dep.resolved, &file).unwrap();
                }
            }
            Err(_err) => {
                println!("file not found: {:?}", file);
                xx::http::download(&dep.resolved, &file).unwrap();
            }
        }
    }
}
