use std::fs::File;
use std::io::{Write, Error};

pub mod packages;
pub mod package;

use crate::packages::Packages;

pub fn run() -> Result<(), Error> {
    let packages = Packages::new().packages;

    let mut r = String::new();

    for p in packages {
        r.push_str(&p.get_root().unwrap().into_string().unwrap());
    }

    let path = "files.txt";
    let mut output = File::create(path)?;
    let _r = write!(output, "{}", r);

    Ok(())
}
