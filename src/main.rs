use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	//if args.len() < 3 {
		//return Err("not enough arguments");
	//}

    let lib_tags_filename = args[1].clone();
    let deps_tags_filename = args[2].clone();
	
	let lib_tags = rexctags::Tag::new(lib_tags_filename).unwrap();
	let deps_tags = rexctags::Tag::new(deps_tags_filename).unwrap();

	if let Err(e) = rexctags::run(lib_tags, deps_tags) {
		println!("oops: {}", e);
		process::exit(1);
	}
}
