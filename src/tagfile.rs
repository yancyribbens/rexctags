use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct TagFile {
    _version: u8, // !_TAG_FILE_FORMAT
    _sorted: u8, // !_TAG_FILE_SORTED
    entries: Vec<String>
}

impl TagFile {
    pub fn new(raw_file_tags: String) -> Result<TagFile, Box<dyn Error>> {
        let entries = TagFile::parse(raw_file_tags).unwrap();
        let _version = 2;
        let _sorted = 1;
        Ok(TagFile { _version, _sorted, entries })
    }

    pub fn is_file_header(line: &String) -> bool {
        if line.len() >= 2 {
            let bytes = line.as_bytes();
            bytes[0] == 33u8 && bytes[1] == 95u8
        } else {
            false
        }
    }

    pub fn parse(raw_tag_file: String) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tags: Vec<String> = Vec::new();
        let lines: Vec<String> = raw_tag_file.split("\n").map(|s| s.to_string()).collect();
        for line in lines {
            if line.len() > 0 && !TagFile::is_file_header(&line) {
                tags.push(line);
            }
        }
        Ok(tags)
    }

    pub fn merge(mut self, mut source_tag: TagFile) -> Result<TagFile, Box<dyn Error>> {
        self.entries.append(&mut source_tag.entries);
        self.entries.sort_by(|m, n| m.cmp(&n));
        self.entries.dedup();
        Ok(self) 
    }

    pub fn write(self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        file.write_all(br#"!_TAG_FILE_FORMAT   2   /extended format; --format=1 will not append ;" to lines/"#)?;
        file.write_all(b"\n");
        file.write_all(br#"!_TAG_FILE_SORTED   1   /0=unsorted, 1=sorted, 2=foldcase/"#);
        file.write_all(b"\n");
        for e in self.entries {
            file.write_all(e.as_bytes());
            file.write_all(b"\n");
        }
        file.sync_all()?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag_file_alpha() -> String {
        let tag_file_format_header = r#"!_TAG_FILE_FORMAT	2   /optional comment/"#;
        let tag_file_sorted_header = r#"!_TAG_FILE_SORTED	1			/0=unsorted, 1=sorted/"#;
        let tag_line_alpha = r#"asdf	sub.cc	/^asdf()$/;"	new_field:some\svalue	file:"#;
        let tag_line_beta = r#"foo_t	sub.h	/^typedef foo_t$/;"	kind:t"#;
        let tag_file = format!("{}\n{}\n{}\n{}\n",
            tag_file_format_header,
            tag_file_sorted_header,
            tag_line_alpha,
            tag_line_beta
        );
        tag_file
    }

    fn tag_file_beta() -> String {
        let tag_file_format_header = r#"!_TAG_FILE_FORMAT	2   /optional comment/"#;
        let tag_file_sorted_header = r#"!_TAG_FILE_SORTED	1			/0=unsorted, 1=sorted/"#;
        let tag_line_alpha = r#"getflag	sub.c	/^getflag(arg)$/;"	kind:f	file:"#;
        let tag_line_beta = r#"foo_t	sub.h	/^typedef foo_t$/;"	kind:t"#;
        let tag_file = format!("{}\n{}\n{}\n{}\n",
            tag_file_format_header,
            tag_file_sorted_header,
            tag_line_alpha,
            tag_line_beta
        );
        tag_file
    }

    #[test]
    fn test_merge() {
        let alpha = TagFile::new(tag_file_alpha()).unwrap();
        let beta = TagFile::new(tag_file_beta()).unwrap();
        let merged = alpha.merge(beta).unwrap();

        assert_eq!(merged.entries[0], r#"asdf	sub.cc	/^asdf()$/;"	new_field:some\svalue	file:"#);
        assert_eq!(merged.entries[1], r#"foo_t	sub.h	/^typedef foo_t$/;"	kind:t"#);
        assert_eq!(merged.entries[2], r#"getflag	sub.c	/^getflag(arg)$/;"	kind:f	file:"#);
    }
}
