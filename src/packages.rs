use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serde_json::json;
use std::process::Command;
use walkdir::WalkDir;
use std::path::Path;
use std::ffi::OsString;
use std::path::PathBuf;

use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    packages: Vec<Package>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    manifest_path: String
}

impl Package {
    pub fn get_root(&self) -> Result<OsString> {
        let mut root = OsString::new();
        let mut path = PathBuf::from(&self.manifest_path);

        if path.pop() {
            root = path.into_os_string();
        } else {
            // TODO this should safely return an error
            // however in principle this should never happen
            panic!("oops!");
        }

        Ok(root)
    }

    //pub fn set_path(mut self, manifest_path: OsString) -> Result<()> {
        //let mut path = PathBuf::from(manifest_path);

        //if path.pop() {
            //self.root = path.into_os_string();
        //} 

        //else {
            //return Err("Invalid cargo path")
        //}

        //Ok(())
    //}

    //pub fn get_package_files(self) -> Result<Vec<String>> {
        //let mut result = Vec::new();

        //for entry in WalkDir::new(self.root)
                //.follow_links(true)
                //.into_iter()
                //.filter_map(|e| e.ok()) {

            //let file = entry.file_name().to_string_lossy();
            
            //if file.ends_with(".rs") {
                //result.push(file.to_string());
            //}
        //}

        //Ok(result)
    //}
}

impl Packages {
    pub fn get_packages() -> Result<Packages> {
        let mut output = Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .output()
            .expect("Failed to retrieve cargo metadata");

        let metadata: String = str::from_utf8(&output.stdout)
            .expect("Unable to convert Cargo Metadata to a string")
            .to_string();

        let p: Packages = serde_json::from_str(&metadata)
            .expect("Unable to create Package from json string");

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    use tempfile::tempdir;
    use std::fs::File;
    use std::io::{self, Write};

    fn packages() -> Packages {
        let package_alpha = Package {
            manifest_path: String::from("/alpha/Cargo.toml"),
        };

        let package_beta = Package {
            manifest_path: String::from("/beta/Cargo.toml"),
        };

        let packages = vec![package_alpha, package_beta];

        let packages = Packages {
            packages: packages,
        };

        packages
    }

    #[test]
    fn test_package_serde() {
        let packages = packages();

        let j = serde_json::to_string(&packages).unwrap();
        let p: Packages = serde_json::from_str(&j).unwrap();
    }

    #[test]
    fn test_package_get_root() {
        let packages = packages().packages;
        let alpha = &packages[0];
        let beta = &packages[1];

        let alpha_root = alpha.get_root().unwrap();
        let beta_root = beta.get_root().unwrap();

        assert_eq!(OsString::from("/alpha"), alpha_root);
        assert_eq!(OsString::from("/beta"), beta_root);
    }

    //#[test]
    //fn test_get_files() {
        //let dir = tempdir().unwrap();
        //let file_path = dir.path().join("lorem_ipsum.rs");
        //let manifest = dir.path().join("Cargo.toml");

        //let file = File::create(file_path).unwrap();
        //let manifest_file = File::create(manifest).unwrap(); 

        //println!("{:?}", dir.path());

        //let manifest_path: String = dir
            //.path()
            //.join("Cargo.toml")
            //.into_os_string()
            //.into_string()
            //.unwrap();

        //let package = Package {
            //manifest_path: manifest_path,
        //};

        //let files = package.get_package_files().unwrap();
        //assert_eq!(files.len(), 1);

        //assert_eq!(files[0], "lorem_ipsum.rs");
    //}
}
