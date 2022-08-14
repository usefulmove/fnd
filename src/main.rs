use::colored::*;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const RELEASE_STATE: &str = "a";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    //println!("{:#?}", args); // debug

    match args[1].as_str() {
        "--help" | "help" => {
            // display command usage information
            show_help();
            return;
        }
        "--version" | "version" => {
            // display version information
            println!("  {}{}", env!("CARGO_PKG_VERSION"), RELEASE_STATE);
            return;
        }
        _ => ()
    }

    // execute fnd core

    // 0.2.0 - simple search
    let search_term: String = args[1].clone();

    for entry in WalkDir::new("./") {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if path_str.contains(&search_term) {
            println!("{}", path_str);
        }
    }

}

fn show_help() {
    //TODO
    println!("  \"( oh! hola. need some halp? )\"");
}
