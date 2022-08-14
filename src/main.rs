use std::env;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

const RELEASE_STATE: &str = "b";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // get command arguments
    let args: Vec<String> = env::args().collect();

    //println!("{:#?}", args); // debug

    if args.len() <= 1 {
        for entry in WalkDir::new("./") {
            let entry: DirEntry = entry.unwrap();
            let path: &Path = entry.path();
            let path_str: &str = path.to_str().unwrap();
            println!("{}", path_str);
        }
        return;
    }

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

    /* fnd core */

    // 0.2.0 - simple search
    let search_term: String = args[1].clone();

    for entry in WalkDir::new("./") {
        let entry: DirEntry = entry.unwrap();
        let path: &Path = entry.path();
        let path_str: &str = path.to_str().unwrap();
        if path_str.contains(&search_term) {
            println!("  {}", path_str);
        }
    }

    std::process::exit(0);
}

fn show_help() {
    //TODO
    println!("  ( oh! hola. need halp? )");
}
