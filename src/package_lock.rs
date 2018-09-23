extern crate flate2;
extern crate serde_json;
extern crate tar;
extern crate xx;

use self::flate2::read::GzDecoder;
use self::tar::Archive;
use self::xx::hash;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLock {
    #[serde(skip_serializing)]
    pub root: Option<PathBuf>,

    pub requires: bool,
    pub lockfile_version: u8,
    pub dependencies: HashMap<String, PackageLockDependency>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLockDependency {
    #[serde(skip_serializing)]
    pub name: Option<String>,

    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub requires: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, PackageLockDependency>>,
}

impl PackageLockDependency {
    pub fn cache_path(&self) -> PathBuf {
        let name = &self.name.as_ref().unwrap();

        Path::new("tmp")
            .join(name)
            .join(format!("{}-{}.tgz", name, &self.version))
    }
}

impl PackageLock {
    pub fn new<T: AsRef<Path>>(root: T) -> PackageLock {
        let root = Path::new(root.as_ref());
        let mut file =
            fs::File::open(&root.join("package-lock.json")).expect("package-lock not found");
        let mut lock: PackageLock =
            serde_json::from_reader(&mut file).expect("invalid package-lock");
        lock.root = Some(root.to_owned());
        println!("package-lock.json version: {}", lock.lockfile_version);

        fn init_dep(dependencies: &mut HashMap<String, PackageLockDependency>) {
            for (name, mut dep) in dependencies {
                dep.name = Some(name.clone());
                match dep.dependencies {
                    Some(ref mut deps) => init_dep(deps),
                    None => (),
                }
            }
        };
        init_dep(&mut lock.dependencies);

        lock
    }

    pub fn install(&self) {
        let root = self.root.as_ref().unwrap();
        install(&root, &self.dependencies);
    }
}

fn install<P: AsRef<Path>>(root: P, dependencies: &HashMap<String, PackageLockDependency>) {
    let root = root.as_ref();
    for (_, dep) in dependencies {
        let name = dep.name.as_ref().unwrap();
        match dep.dependencies {
            Some(ref deps) => {
                let mut p = root.join("node_modules");
                p.push(name);
                install(p, deps);
            }
            None => (),
        }
        let file = dep.cache_path();
        match verify(&file, &dep.integrity, false) {
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
        verify(&file, &dep.integrity, true).unwrap();
        let extract_path = root.join("node_modules").join(dep.name.clone().unwrap());
        extract(&file, extract_path);
    }
}

fn verify(path: &Path, integrity: &String, must: bool) -> Result<bool, io::Error> {
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

fn extract<A: AsRef<Path>, B: AsRef<Path>>(tarball: A, to: B) {
    let tarball = tarball.as_ref();
    let to = to.as_ref();
    fn get_real_path(parent: &Path, child: &Path) -> PathBuf {
        let orig_path = child;
        let path = parent.join(orig_path.strip_prefix("package").unwrap());
        if !path.starts_with(parent) {
            panic!("invalid tarball");
        }

        path
    }
    println!("unpacking {:?}", tarball);
    let file = GzDecoder::new(File::open(tarball).unwrap());
    let mut archive = Archive::new(file);
    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        let path = get_real_path(to, file.path().unwrap().as_ref());
        println!("{:?}", path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut output = File::create(&path).unwrap();
        io::copy(&mut file, &mut output).unwrap();
    }
}
