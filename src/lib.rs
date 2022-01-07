use std::fs;
use std::error::Error;
use serde_json::{Result, Value};

pub mod tagfile;

use crate::tagfile::TagFile;
use std::process::Command;

pub fn dep_paths() -> Result<()> {
    let mut output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Oops, failed to generate cargo metada");

    let result = String::from_utf8_lossy(&output.stdout);
    let v: Value = serde_json::from_str(&result)?;

    Ok(())
}

pub fn run() -> Result<()> {
    dep_paths();
    Ok(())
}

//pub fn run(tag_filename_alpha: String, tag_filename_beta: String) -> Result<(), Box<dyn Error>> {
    //let raw_alpha_file = fs::read_to_string(&tag_filename_alpha).unwrap();
    //let alpha_tag_file = TagFile::new(raw_alpha_file).unwrap();

    //let raw_beta_file = fs::read_to_string(tag_filename_beta).unwrap();
    //let beta_tag_file = TagFile::new(raw_beta_file).unwrap();

    //let alpha_tag_file = alpha_tag_file.merge(beta_tag_file).unwrap();
    //alpha_tag_file.write("new_tagfile.vi");

    //Ok(())
//}
