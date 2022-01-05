use std::error::Error;

// {tagname}<Tab>{tagfile}<Tab>{tagaddress}[;"<Tab>{tagfield}..]
pub struct Tag {
    pub tagname: String,
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

#[cfg(test)]
mod tests {
    use super::*;

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

}
