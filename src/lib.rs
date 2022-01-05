use std::fs;
use std::error::Error;

pub mod tagfile;
pub mod tag;

use crate::tagfile::TagFile;

pub fn run(lib_tag_filename: String, dep_tag_filename: String) -> Result<(), Box<dyn Error>> {
    let raw_lib_tag_file = fs::read_to_string(lib_tag_filename).unwrap();
    let lib_tag_file = TagFile::new(raw_lib_tag_file);
    //let tags: Vec<String> = tags_file.split("\n").map(|s| s.to_string()).collect();
    Ok(())
}
