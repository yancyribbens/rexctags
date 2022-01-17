use std::process;

fn main() {
    if let Err(e) = rexctags::run() {
        println!("oops: {}", e);
        process::exit(1);
    }
}
