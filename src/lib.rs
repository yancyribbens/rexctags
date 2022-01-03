//use std::fs;
use std::error::Error;

//pub fn run(lib_tag_file: Tag, dep_tag_file: Tag) -> Result<(), Box<dyn Error>> {
    //let tags_file = fs::read_to_string(lib_tags_filename).unwrap();
    //let tags: Vec<String> = tags_file.split("\n").map(|s| s.to_string()).collect();
    //Ok(())
//}

// {tagname}<Tab>{tagfile}<Tab>{tagaddress}[;"<Tab>{tagfield}..]
pub struct Tag {
    tagname: String,
    _tagfile: Option<String>,
    _tagaddress: Option<String>,
    _tagfield: Option<String>
}

// http://ctags.sourceforge.net/FORMAT
impl Tag {
    pub fn new(tag_line: String) -> Result<Tag, Box<dyn Error>> {
        // {tagname}    Any identifier, not containing white space..
        // <Tab> Exactly one TAB character (although many versions of Vi can handle any amount of white space).
        let mut iter = tag_line.split_whitespace();
        let tagname: String = String::from(iter.next().unwrap());

        let _tagfile = None;
        let _tagaddress = None;
        let _tagfield = None; 

        Ok(Tag { tagname, _tagfile, _tagaddress, _tagfield })
    }
}

pub struct TagFile {
    _version: u8, // !_TAG_FILE_FORMAT
    _sorted: u8, // !_TAG_FILE_SORTED
    tags: Vec<Tag>
}

impl TagFile {
    pub fn new(tags: Vec<Tag>) -> Result<TagFile, Box<dyn Error>> {
        let _version = 2;
        let _sorted = 1;
        Ok(TagFile { _version, _sorted, tags })
    }

    pub fn is_file_header(line: &String) -> bool {
        line.as_bytes()[0] == 33u8
    }

    pub fn parse(raw_tag_file: String) -> Result<Vec<Tag>, Box<dyn Error>> {
        //let tags_file = fs::read_to_string(&self.location)?;
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

    #[test]
    fn test_tag_file_parse() {
        let raw_tag_file = tag_file_alpha();
        println!("tagfile: {}", raw_tag_file);
        let tag_file = TagFile::parse(raw_tag_file).unwrap();

        let tagname_alpha: String = tag_file[0].tagname.clone();
        let tagname_beta: String = tag_file[1].tagname.clone();
        
        assert_eq!("asdf", tagname_alpha);
        assert_eq!("inc", tagname_beta);
    }

    #[test]
    fn test_tag_parse() {
        let tag_line = String::from(r#"asdf	sub.cc  /^asdf()$/;"    new_field:some\svalue   file:"#);
        let tag = Tag::new(tag_line).unwrap();
        assert_eq!(tag.tagname, "asdf");

        // TODO
        //assert_eq!(tag.tagfile, "sub.cc");
        //assert_eq!(tag.tagaddress, "^asdf()$/;");
        //assert_eq!(tag.tagfield, vec![r#"new_field:some\svalue"#, "file:"]);
    }

    #[test]
    fn test_tag_file() {
    }
}
