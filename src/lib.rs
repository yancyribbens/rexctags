#[cfg(test)]
use mockall::{automock, mock, predicate::*};

use std::fs;
use std::error::Error;

pub fn run(lib_tag_file: Tag, dep_tag_file: Tag) -> Result<(), Box<dyn Error>> {
    //let tags_file = fs::read_to_string(lib_tags_filename).unwrap();
    //let tags: Vec<String> = tags_file.split("\n").map(|s| s.to_string()).collect();
    Ok(())
}

pub struct Tag {
    version: u8, // !_TAG_FILE_FORMAT
    sorted: u8, // !_TAG_FILE_SORTED
	location: String,
}

impl Tag {
	pub fn new(filename: String) -> Result<Tag, Box<dyn Error>> {
		let version = 2;
		let sorted = 1;
        let location = filename;
		Ok(Tag { version, sorted, location })
	}

    pub fn parse(self) -> Result<Vec<String>, Box<dyn Error>> {
        let tags_file = fs::read_to_string(self.location)?;
        let tags: Vec<String> = tags_file.split("\n").map(|s| s.to_string()).collect();
        Ok(tags)
    }

    //TODO read source tags, merge, sort and dedup (and test)
    //pub fn merge(source_tags) {
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
		let a = vec!["a", "b"];

        assert_eq!(a, a);
    }
}
