use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serde_json::json;
use std::process::Command;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    packages: Vec<Package>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    manifest_path: String
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

    #[test]
    fn test_metadata() {
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

        let j = serde_json::to_string(&packages).unwrap();
        let p: Packages = serde_json::from_str(&j).unwrap();

        assert_eq!(p.packages[0].manifest_path, String::from("/alpha/Cargo.toml"));
        assert_eq!(p.packages[1].manifest_path, String::from("/beta/Cargo.toml"));
    }
}
