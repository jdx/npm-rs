extern crate xx;

use extract::verify_tarball;
use package_lock::PackageLock;

pub fn download_tarballs(lock: &PackageLock) {
    for (name, dep) in &lock.dependencies {
        let file = dep.cache_path();
        match verify_tarball(&file, &dep.integrity) {
            Ok(verified) => {
                if !verified {
                    println!("hash fail: {:?}", file);
                    xx::http::download(&dep.resolved, &file).unwrap();
                }
            }
            Err(err) => {
                println!("file not found: {:?}", file);
                xx::http::download(&dep.resolved, &file).unwrap();
            }
        }
    }
}
