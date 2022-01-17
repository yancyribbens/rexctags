use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::Command;

use crate::package::Package;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    pub packages: Vec<Package>
}

impl Packages {
    pub fn new() -> Packages {
        Packages::get_packages().unwrap()
    }

    pub fn get_packages() -> Result<Packages> {
        let output = Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .output()
            .expect("Failed to retrieve cargo metadata");

        let metadata: String = std::str::from_utf8(&output.stdout)
            .expect("Unable to convert Cargo Metadata to a string")
            .to_string();

        let p: Packages = serde_json::from_str(&metadata)
            .expect("Unable to create Package from json string");

        Ok(p)
    }
}
