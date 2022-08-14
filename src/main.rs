use std::env;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

mod poc;

const RELEASE_STATE: &str = "d";

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
        "--help" => {
            // display command usage information
            show_help();
            return;
        }
        "--version" => {
            // display version information
            println!(
                "  {} {}{}",
                poc::color_grey_mouse("fnd"),
                poc::color_blue_smurf_bold(env!("CARGO_PKG_VERSION")),
                poc::color_white_bold(RELEASE_STATE),
            );
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
            //println!("  {}", path_str); // debug
            println!(
                "  {}",
                //poc::highlight(path_str, &search_term),
                poc::highlight(
                    path_str,
                    &search_term
                ),
            );
        }
    }

    std::process::exit(0);
}

fn show_help() {
    //TODO
    println!("  ( oh! hola. need halp? )");
}