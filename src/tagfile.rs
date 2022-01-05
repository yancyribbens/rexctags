use std::error::Error;
use crate::tag::Tag;

pub struct TagFile {
    _version: u8, // !_TAG_FILE_FORMAT
    _sorted: u8, // !_TAG_FILE_SORTED
    tags: Vec<Tag>
}

impl TagFile {
    pub fn new(raw_file_tags: String) -> Result<TagFile, Box<dyn Error>> {
        let tags = TagFile::parse(raw_file_tags).unwrap();
        let _version = 2;
        let _sorted = 1;
        Ok(TagFile { _version, _sorted, tags })
    }

    pub fn is_file_header(line: &String) -> bool {
        line.as_bytes()[0] == 33u8
    }

    pub fn parse(raw_tag_file: String) -> Result<Vec<Tag>, Box<dyn Error>> {
        let mut tags: Vec<Tag> = Vec::new();
        let lines: Vec<String> = raw_tag_file.split("\n").map(|s| s.to_string()).collect();
        for line in lines {
            if !TagFile::is_file_header(&line) {
                let tag = Tag::new(line).unwrap(); 
                tags.push(tag);
            }
        }
        Ok(tags)
    }

    //TODO dedup (and test)
    pub fn merge(mut self, mut source_tag: TagFile) -> Result<(), Box<dyn Error>> {
        self.tags.append(&mut source_tag.tags);
        self.tags.sort_by(|m, n| m.tagname.cmp(&n.tagname));
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
        let tag_line_beta = r#"inc	sub.cc	/^inc()$/;"	file: class:PipeBuf\n"#;
        let tag_file = format!("{}\n{}\n{}\n{}",
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
        let tag_line_alpha = r#"foo_t	sub.h	/^typedef foo_t$/;"	kind:t"#;
        let tag_line_beta = r#"func3	sub.p	/^func3()$/;"	function:/func1/func2	file:"#;
		let tag_line_gamma = r#"getflag	sub.c	/^getflag(arg)$/;"	kind:f	file:"#;
        let tag_file = format!("{}\n{}\n{}\n{}\n{}",
            tag_file_format_header,
            tag_file_sorted_header,
            tag_line_alpha,
            tag_line_beta,
			tag_line_gamma
		);
		tag_file
    }

    #[test]
    fn test_tag_file() {
        let raw_tag_file = tag_file_alpha();
        let tagfile = TagFile::new(raw_tag_file).unwrap();
        assert_eq!("asdf", tagfile.tags[0].tagname);
        assert_eq!("inc", tagfile.tags[1].tagname);
    }

}
