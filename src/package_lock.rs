extern crate serde_json;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::convert::AsRef;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLockDependency {
    #[serde(skip_serializing)]
    name: Option<String>,

    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub requires: Option<HashMap<String, String>>,
}

impl PackageLockDependency {
    pub fn cache_path(&self) -> PathBuf {
        let name = &self.name.as_ref().unwrap();

        Path::new("tmp")
            .join(name)
            .join(format!("{}-{}.tgz", name, &self.version))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLock {
    pub requires: bool,
    pub lockfile_version: u8,
    pub dependencies: HashMap<String, PackageLockDependency>,
}

impl PackageLock {
    pub fn new<T: AsRef<Path>>(root: T) -> PackageLock {
        let path = Path::new(root.as_ref()).join("package-lock.json");
        let mut file = fs::File::open(path).expect("package-lock not found");
        let mut lock: PackageLock = serde_json::from_reader(&mut file).expect("invalid package-lock");
        println!("package-lock.json version: {}", lock.lockfile_version);

        for (name, mut dep) in &mut lock.dependencies {
            dep.name = Some(name.clone());
        }

        lock
    }
}
