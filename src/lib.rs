use std::fs;
use std::error::Error;

pub mod tagfile;

use crate::tagfile::TagFile;

pub fn run(tag_filename_alpha: String, tag_filename_beta: String) -> Result<(), Box<dyn Error>> {
    let raw_alpha_file = fs::read_to_string(&tag_filename_alpha).unwrap();
    let alpha_tag_file = TagFile::new(raw_alpha_file).unwrap();

    let raw_beta_file = fs::read_to_string(tag_filename_beta).unwrap();
    let beta_tag_file = TagFile::new(raw_beta_file).unwrap();

    let alpha_tag_file = alpha_tag_file.merge(beta_tag_file).unwrap();
    alpha_tag_file.write("new_tagfile.vi");

    Ok(())
}
